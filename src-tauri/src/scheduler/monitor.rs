// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Background Monitor (Phase 1)
//
// Central Tokio loop that:
// 1. Polls idle time from platform provider every N seconds
// 2. Evaluates activity state using domain logic
// 3. Updates shared AppState
// 4. Emits Tauri events to the frontend
// ────────────────────────────────────────────────────────────────────────────
use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::RwLock;

use crate::application::activity_service::ActivityService;
use crate::domain::activity::{evaluate_state, ActivityState, CurrentActivity};
use crate::domain::break_rules::{compute_countdowns, BreakConfig};
use crate::infrastructure::platform::PlatformProvider;

/// Payload emitted with `activity://changed` event.
#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityChangedPayload {
    pub state: ActivityState,
    pub idle_seconds: u64,
    pub active_session_seconds: u64,
}

/// Run the central monitor loop.
/// This should be spawned as a Tokio task and run for the lifetime of the app.
pub async fn run_monitor(
    app_handle: AppHandle,
    platform: Arc<dyn PlatformProvider + Send + Sync>,
    current_activity: Arc<RwLock<CurrentActivity>>,
    monitoring_paused: Arc<RwLock<bool>>,
    poll_interval_seconds: u64,
    idle_cutoff_seconds: u64,
    break_config: BreakConfig,
) {
    tracing::info!(
        "Background monitor started (poll={}s, idle_cutoff={}s)",
        poll_interval_seconds,
        idle_cutoff_seconds
    );

    let mut active_session_seconds: u64 = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(poll_interval_seconds)).await;

        // ── Check if paused
        if *monitoring_paused.read().await {
            tracing::trace!("Monitor: paused");
            continue;
        }

        // ── Get idle time from platform
        let idle_seconds = match platform.idle_seconds().await {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Monitor: idle_seconds error: {e}");
                0 // treat as active if detection fails
            }
        };

        // ── Evaluate activity state
        let (prev_state, stored_active_seconds) = {
            let a = current_activity.read().await;
            (a.state.clone(), a.active_session_seconds)
        };

        if stored_active_seconds == 0 && active_session_seconds > 0 {
            active_session_seconds = 0;
        }

        let mut new_state = evaluate_state(idle_seconds, idle_cutoff_seconds, &prev_state);

        // ── Accumulate active session seconds
        if new_state == ActivityState::Active {
            active_session_seconds += poll_interval_seconds;
        } else if matches!(new_state, ActivityState::Idle | ActivityState::Breaking) {
            // Reset active accumulation when truly idle (sufficient idle for break)
            if idle_seconds >= break_config.auto_complete_idle_seconds {
                active_session_seconds = 0;
                new_state = ActivityState::Active; // Or let it be Idle, but active accumulation is reset
            }
        }

        // ── Check if break is due
        let break_due = crate::domain::break_rules::evaluate_break_due(active_session_seconds, &break_config);
        if new_state == ActivityState::Active && break_due != crate::domain::break_rules::BreakDue::None {
            new_state = ActivityState::BreakDue;
        }

        if prev_state != ActivityState::BreakDue && new_state == ActivityState::BreakDue {
            // Transitioned to BreakDue
            let app_state = app_handle.state::<crate::state::AppState>();
            let mut urgency_prefix = "";
            let mut urgency_suffix = "";
            let mut habit_score = 100;

            if let Ok(stats) = crate::infrastructure::database::repositories::get_today_statistics(&app_state.db).await {
                if stats.skipped_break_count > 2 || stats.snoozed_reminder_count > 3 {
                    urgency_prefix = "⚠️ [URGENT] ";
                    urgency_suffix = " You have been ignoring breaks today. Your body needs this!";
                }
                habit_score = stats.desk_habit_score.unwrap_or(100);
            }

            let (mut title, mut body) = match break_due {
                crate::domain::break_rules::BreakDue::Eye => {
                    let mut b = "Look away from the screen for about 20 seconds. (Eye Palming recommended)".to_string();
                    if habit_score < 50 { b.push_str(" You're straining your eyes lately, please take this seriously!"); }
                    ("Eye break".to_string(), b)
                },
                crate::domain::break_rules::BreakDue::Short => {
                    let stretches = ["Neck Stretch", "Wrist Extension", "Seated Twist", "Shoulder Shrug"];
                    let random_stretch = stretches[(active_session_seconds % 4) as usize];
                    let mut b = format!("You've been active for a while. A short break may be useful. Recommended stretch: {random_stretch}");
                    if habit_score < 70 { b.push_str(" A longer short-break is highly recommended due to recent habits."); }
                    ("Time to move".to_string(), b)
                },
                crate::domain::break_rules::BreakDue::Long => {
                    let mut b = "You've been active for a long time. A longer break is recommended.".to_string();
                    if habit_score < 80 { b.push_str(" Step away from the desk completely."); }
                    ("Time to move".to_string(), b)
                },
                _ => ("".to_string(), "".to_string()),
            };

            if break_due != crate::domain::break_rules::BreakDue::None {
                title = format!("{}{}", urgency_prefix, title);
                body = format!("{}{}", body, urgency_suffix);
                crate::application::break_service::BreakService::send_break_notification(&app_handle, &format!("{:?}", break_due).to_lowercase(), &title, &body);
                
                let _ = crate::application::break_service::BreakService::record_reminder_event(
                    &app_state.db,
                    None,
                    &format!("{:?}", break_due).to_lowercase(),
                    active_session_seconds,
                    "triggered"
                ).await;
            }
        }

        // ── Compute break countdowns
        let countdowns = compute_countdowns(active_session_seconds, &break_config);

        // ── Update shared state
        {
            let mut activity = current_activity.write().await;
            activity.state = new_state.clone();
            activity.idle_seconds = idle_seconds;
            activity.active_session_seconds = active_session_seconds;
            activity.next_eye_break_seconds = countdowns.eye;
            activity.next_short_break_seconds = countdowns.short;
            activity.next_long_break_seconds = countdowns.long;
            activity.last_updated_utc = chrono::Utc::now();
        }

        // ── Emit event to frontend
        let payload = ActivityChangedPayload {
            state: new_state.clone(),
            idle_seconds,
            active_session_seconds,
        };

        if let Err(e) = app_handle.emit("activity://changed", payload) {
            tracing::warn!("Monitor: failed to emit activity://changed: {e}");
        }

        // ── Check foreground application (Phase 8)
        let app_state = app_handle.state::<crate::state::AppState>();
        
        let track_enabled = match crate::infrastructure::database::repositories::get_setting(&app_state.db, "privacy.track_foreground_app").await {
            Ok(Some(v)) => v == "true",
            _ => false,
        };

        let foreground_app = if track_enabled {
            match platform.foreground_app().await {
                Ok(app) => app,
                Err(e) => {
                    tracing::trace!("Monitor: foreground_app error: {e}");
                    None
                }
            }
        } else {
            None
        };

        // ── Persist to database (Phase 2 & 8)
        {
            if let Err(e) = ActivityService::record_activity(&app_state.db, &new_state, poll_interval_seconds, foreground_app).await {
                tracing::error!("Monitor: failed to record activity: {:?}", e);
            }
        }

        tracing::trace!(
            "Monitor: state={:?} idle={}s active={}s",
            new_state,
            idle_seconds,
            active_session_seconds
        );
    }
}

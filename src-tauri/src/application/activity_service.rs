// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Activity Service
// ────────────────────────────────────────────────────────────────────────────
use chrono::Utc;
use sqlx::SqlitePool;

use crate::domain::activity::ActivityState;
use crate::errors::AppError;
use crate::infrastructure::database::repositories;

pub struct ActivityService;

impl ActivityService {
    /// Called on app startup to clean up any open segments and sessions
    /// from a previous crash or ungraceful shutdown.
    pub async fn recover_crash_state(pool: &SqlitePool) -> Result<(), AppError> {
        let segments_closed = repositories::close_stale_segments(pool).await?;
        let sessions_closed = repositories::close_stale_sessions(pool).await?;
        
        if segments_closed > 0 || sessions_closed > 0 {
            tracing::info!(
                "Crash recovery: closed {} stale segments and {} stale sessions",
                segments_closed,
                sessions_closed
            );
        }
        
        Ok(())
    }

    /// Records the current activity state, merging with the previous segment if the state is the same,
    /// and managing the active work session.
    pub async fn record_activity(
        pool: &SqlitePool,
        state: &ActivityState,
        duration_seconds: u64,
        app: Option<crate::infrastructure::platform::ForegroundApp>,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        let state_str = state.to_string().to_lowercase();
        
        let app_name = app.as_ref().map(|a| a.name.clone());
        let app_identifier = app.as_ref().and_then(|a| a.identifier.clone());
        
        // 1. Handle Activity Segments
        let latest_segment = repositories::get_latest_segment(pool).await?;
        
        if let Some(segment) = latest_segment {
            let same_app = segment.app_identifier == app_identifier;
            
            if segment.state == state_str && segment.ended_at_utc.is_none() && same_app {
                // Same state, same app, and still open, just extend it
                repositories::extend_segment(pool, segment.id, now.to_rfc3339(), duration_seconds as i64).await?;
            } else {
                // State changed, app changed, or previous was closed.
                if segment.ended_at_utc.is_none() {
                    // Close the previous one if it was open
                    repositories::close_segment(pool, segment.id, now.to_rfc3339()).await?;
                }
                
                // Create new open segment
                repositories::insert_segment(pool, &state_str, now.to_rfc3339(), app_name, app_identifier).await?;
            }
        } else {
            // No segments exist at all
            repositories::insert_segment(pool, &state_str, now.to_rfc3339(), app_name, app_identifier).await?;
        }

        // 2. Handle Work Sessions
        let active_session = repositories::get_active_work_session(pool).await?;
        
        match state {
            ActivityState::Active | ActivityState::Idle | ActivityState::BreakDue => {
                if let Some(session) = active_session {
                    let active_add = if *state == ActivityState::Active || *state == ActivityState::BreakDue { duration_seconds as i64 } else { 0 };
                    let idle_add = if *state == ActivityState::Idle { duration_seconds as i64 } else { 0 };
                    
                    repositories::extend_work_session(pool, session.id, now.to_rfc3339(), active_add, idle_add).await?;
                } else {
                    // Start a new session if active
                    if *state == ActivityState::Active {
                        repositories::insert_work_session(pool, now.to_rfc3339()).await?;
                    }
                }
            }
            ActivityState::Locked | ActivityState::Suspended => {
                // These interrupt a session, wait actually maybe they just pause it? 
                // The requirements: end_reason in ('break', 'manual', 'locked', 'suspended', 'shutdown', 'app_restart')
                if let Some(session) = active_session {
                    repositories::close_work_session(pool, session.id, now.to_rfc3339(), &state_str).await?;
                }
            }
            _ => {}
        }

        // 3. Update Daily Statistics
        let active_add = if *state == ActivityState::Active || *state == ActivityState::BreakDue { duration_seconds as i64 } else { 0 };
        let idle_add = if *state == ActivityState::Idle { duration_seconds as i64 } else { 0 };
        let break_add = if *state == ActivityState::Breaking { duration_seconds as i64 } else { 0 };
        
        repositories::increment_daily_statistics(pool, active_add, idle_add, break_add).await?;

        Ok(())
    }
}

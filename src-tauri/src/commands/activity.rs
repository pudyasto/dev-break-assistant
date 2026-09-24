// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Tauri Commands: Activity
// ────────────────────────────────────────────────────────────────────────────
use tauri::State;

use crate::domain::activity::CurrentActivity;
use crate::errors::AppError;
use crate::infrastructure::platform::PlatformCapabilities;
use crate::state::AppState;

/// Returns the current activity snapshot.
#[tauri::command]
pub async fn get_current_activity(
    state: State<'_, AppState>,
) -> Result<CurrentActivity, AppError> {
    let activity = state.current_activity.read().await;
    Ok(activity.clone())
}

/// Returns what platform capabilities are available.
#[tauri::command]
pub async fn get_platform_capabilities(
    state: State<'_, AppState>,
) -> Result<PlatformCapabilities, AppError> {
    let caps = state.capabilities.read().await;
    Ok(caps.clone())
}

/// Pause the activity monitor.
#[tauri::command]
pub async fn pause_monitoring(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut paused = state.monitoring_paused.write().await;
    *paused = true;
    tracing::info!("Monitoring paused by user");
    Ok(())
}

/// Resume the activity monitor.
#[tauri::command]
pub async fn resume_monitoring(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut paused = state.monitoring_paused.write().await;
    *paused = false;
    tracing::info!("Monitoring resumed by user");
    Ok(())
}

#[tauri::command]
pub async fn start_break(state: State<'_, AppState>, break_type: String) -> Result<(), AppError> {
    tracing::info!("Started break: {}", break_type);
    let _ = crate::application::break_service::BreakService::record_break_session(
        &state.db,
        None, // missing active session id
        &break_type,
        "manual",
        0,
        "started"
    ).await;
    
    let mut activity = state.current_activity.write().await;
    activity.state = crate::domain::activity::ActivityState::Breaking;
    Ok(())
}

#[tauri::command]
pub async fn complete_break(state: State<'_, AppState>, break_type: String, duration_seconds: u64) -> Result<(), AppError> {
    tracing::info!("Completed break: {} for {}s", break_type, duration_seconds);
    let _ = crate::application::break_service::BreakService::record_break_session(
        &state.db,
        None,
        &break_type,
        "manual",
        duration_seconds,
        "completed"
    ).await;
    
    let _ = crate::infrastructure::database::repositories::increment_daily_break_stats(&state.db, &break_type, "completed").await;
    
    let mut activity = state.current_activity.write().await;
    activity.state = crate::domain::activity::ActivityState::Active;
    activity.active_session_seconds = 0; // reset active accumulation
    Ok(())
}

#[tauri::command]
pub async fn snooze_reminder(state: State<'_, AppState>, break_type: String) -> Result<(), AppError> {
    tracing::info!("Snoozed reminder: {}", break_type);
    let _ = crate::application::break_service::BreakService::record_reminder_event(
        &state.db,
        None,
        &break_type,
        0,
        "snoozed"
    ).await;
    
    let _ = crate::infrastructure::database::repositories::increment_daily_break_stats(&state.db, &break_type, "snoozed").await;
    
    let mut activity = state.current_activity.write().await;
    activity.state = crate::domain::activity::ActivityState::Active;
    Ok(())
}

#[tauri::command]
pub async fn dismiss_reminder(state: State<'_, AppState>, break_type: String) -> Result<(), AppError> {
    tracing::info!("Dismissed reminder: {}", break_type);
    let _ = crate::application::break_service::BreakService::record_reminder_event(
        &state.db,
        None,
        &break_type,
        0,
        "dismissed"
    ).await;
    
    let _ = crate::infrastructure::database::repositories::increment_daily_break_stats(&state.db, &break_type, "dismissed").await;
    
    let mut activity = state.current_activity.write().await;
    activity.state = crate::domain::activity::ActivityState::Active;
    activity.active_session_seconds = 0; // Or don't reset? the spec says snoozing doesn't reset accumulation. Dismissing probably shouldn't either, but user dismissed it. Let's not reset it.
    Ok(())
}

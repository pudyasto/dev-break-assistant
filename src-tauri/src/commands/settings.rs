// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Tauri Commands: Settings
// ────────────────────────────────────────────────────────────────────────────
use tauri::{AppHandle, Emitter, State};

use crate::application::settings_service::{AppSettings, SettingsService};
use crate::errors::AppError;
use crate::infrastructure::database::repositories;
use crate::scheduler::monitor::ActivityChangedPayload;
use crate::state::AppState;

/// Retrieve all settings.
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, AppError> {
    SettingsService::get_settings(&state.db).await
}

/// Update a single setting by key.
#[tauri::command]
pub async fn update_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    SettingsService::update_setting(&state.db, &key, &value).await
}

/// Clear all recorded activity logs, sessions, reminders, and daily statistics.
#[tauri::command]
pub async fn reset_all_data(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    repositories::clear_all_logs(&state.db).await?;

    let payload = {
        let mut activity = state.current_activity.write().await;
        activity.active_session_seconds = 0;
        ActivityChangedPayload {
            state: activity.state.clone(),
            idle_seconds: activity.idle_seconds,
            active_session_seconds: 0,
        }
    };

    let _ = app_handle.emit("activity://changed", payload);
    let _ = app_handle.emit("statistics://updated", ());
    Ok(())
}


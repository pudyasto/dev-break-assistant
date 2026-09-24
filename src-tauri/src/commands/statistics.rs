// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Tauri Commands: Statistics
// ────────────────────────────────────────────────────────────────────────────
use tauri::State;

use crate::application::statistics_service::StatisticsService;
use crate::domain::session::WorkSession;
use crate::domain::statistics::TodayStatistics;
use crate::errors::AppError;
use crate::state::AppState;

/// Retrieve today's statistics.
#[tauri::command]
pub async fn get_today_statistics(
    state: State<'_, AppState>,
) -> Result<TodayStatistics, AppError> {
    StatisticsService::get_today(&state.db).await
}

/// Retrieve recent work sessions.
#[tauri::command]
pub async fn get_recent_sessions(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<WorkSession>, AppError> {
    let limit = limit.unwrap_or(10).min(100);
    StatisticsService::get_recent_sessions(&state.db, limit).await
}

/// Retrieve statistics for a date range.
#[tauri::command]
pub async fn get_statistics_range(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<TodayStatistics>, AppError> {
    StatisticsService::get_statistics_range(&state.db, &start_date, &end_date).await
}

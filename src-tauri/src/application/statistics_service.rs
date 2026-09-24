// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Statistics Service (Phase 0 stub, Phase 5 full implementation)
// ────────────────────────────────────────────────────────────────────────────
use sqlx::SqlitePool;

use crate::domain::statistics::TodayStatistics;
use crate::domain::session::WorkSession;
use crate::errors::AppError;
use crate::infrastructure::database::repositories;

pub struct StatisticsService;

impl StatisticsService {
    pub async fn get_today(pool: &SqlitePool) -> Result<TodayStatistics, AppError> {
        repositories::get_today_statistics(pool).await
    }

    pub async fn get_statistics_range(
        pool: &SqlitePool,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<TodayStatistics>, AppError> {
        repositories::get_statistics_range(pool, start_date, end_date).await
    }

    pub async fn get_recent_sessions(
        pool: &SqlitePool,
        limit: i64,
    ) -> Result<Vec<WorkSession>, AppError> {
        repositories::get_recent_sessions(pool, limit).await
    }
}

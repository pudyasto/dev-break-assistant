// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Database Migrations
// ────────────────────────────────────────────────────────────────────────────
use sqlx::SqlitePool;
use crate::errors::AppError;

/// Run all pending SQLx migrations embedded from the migrations/ directory.
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), AppError> {
    tracing::info!("Running database migrations…");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(sqlx::Error::from(e)))?;

    tracing::info!("Database migrations complete");
    Ok(())
}

use sqlx::SqlitePool;
use chrono::Utc;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use crate::errors::AppError;

pub struct BreakService;

impl BreakService {
    pub fn send_break_notification(app: &AppHandle, _break_type: &str, title: &str, body: &str) {
        if let Err(e) = app.notification()
            .builder()
            .title(title)
            .body(body)
            .show()
        {
            tracing::warn!("Failed to show notification: {}", e);
        }
    }

    pub async fn record_reminder_event(
        db: &SqlitePool,
        work_session_id: Option<i64>,
        break_type: &str,
        active_seconds: u64,
        action: &str,
    ) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let active = active_seconds as i64;
        sqlx::query(
            r#"
            INSERT INTO reminder_events (
                work_session_id, break_type, triggered_at_utc,
                active_seconds_at_trigger, action, action_at_utc, created_at_utc
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(work_session_id)
        .bind(break_type)
        .bind(&now)
        .bind(active)
        .bind(action)
        .bind(&now)
        .bind(&now)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn record_break_session(
        db: &SqlitePool,
        work_session_id: Option<i64>,
        break_type: &str,
        source: &str,
        duration_seconds: u64,
        status: &str,
    ) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let dur = duration_seconds as i64;
        sqlx::query(
            r#"
            INSERT INTO break_sessions (
                work_session_id, break_type, source,
                started_at_utc, duration_seconds, status, created_at_utc
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(work_session_id)
        .bind(break_type)
        .bind(source)
        .bind(&now)
        .bind(dur)
        .bind(status)
        .bind(&now)
        .execute(db)
        .await?;

        Ok(())
    }
}


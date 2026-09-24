// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Database Repositories
// All SQL queries are centralized here.
// ────────────────────────────────────────────────────────────────────────────
use chrono::Utc;
use sqlx::{SqlitePool, Row};

use crate::domain::statistics::TodayStatistics;
use crate::domain::session::{WorkSession, WorkSessionStatus};
use crate::errors::AppError;

// ─── Settings ─────────────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
pub struct SettingRow {
    pub key: String,
    pub value: String,
    pub value_type: String,
}

pub async fn get_all_settings(pool: &SqlitePool) -> Result<Vec<SettingRow>, AppError> {
    let rows = sqlx::query_as::<_, SettingRow>(
        "SELECT key, value, value_type FROM app_settings ORDER BY key"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn upsert_setting(
    pool: &SqlitePool,
    key: &str,
    value: &str,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO app_settings (key, value, value_type, updated_at_utc)
         VALUES (?1, ?2, 'string', ?3)
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at_utc = ?3"
    )
    .bind(key)
    .bind(value)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_setting(pool: &SqlitePool, key: &str) -> Result<Option<String>, AppError> {
    let row = sqlx::query("SELECT value FROM app_settings WHERE key = ?1")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.get::<String, _>("value")))
}

// ─── Daily Statistics ──────────────────────────────────────────────────────────

pub async fn get_today_statistics(pool: &SqlitePool) -> Result<TodayStatistics, AppError> {
    // Get local date in user's timezone
    let local_date = chrono::Local::now().format("%Y-%m-%d").to_string();

    let row = sqlx::query(
        r#"SELECT
            local_date,
            active_seconds,
            idle_seconds,
            break_seconds,
            break_count,
            eye_break_count,
            short_break_count,
            long_break_count,
            skipped_break_count,
            snoozed_reminder_count,
            longest_active_streak_seconds,
            desk_habit_score
        FROM daily_statistics
        WHERE local_date = ?1"#
    )
    .bind(&local_date)
    .fetch_optional(pool)
    .await?;

    Ok(match row {
        Some(r) => TodayStatistics {
            local_date: r.get("local_date"),
            active_seconds: r.get("active_seconds"),
            idle_seconds: r.get("idle_seconds"),
            break_seconds: r.get("break_seconds"),
            break_count: r.get("break_count"),
            eye_break_count: r.get("eye_break_count"),
            short_break_count: r.get("short_break_count"),
            long_break_count: r.get("long_break_count"),
            skipped_break_count: r.get("skipped_break_count"),
            snoozed_reminder_count: r.get("snoozed_reminder_count"),
            longest_active_streak_seconds: r.get("longest_active_streak_seconds"),
            desk_habit_score: r.get("desk_habit_score"),
        },
        None => TodayStatistics {
            local_date,
            ..Default::default()
        },
    })
}

pub async fn get_statistics_range(
    pool: &SqlitePool,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<TodayStatistics>, AppError> {
    let rows = sqlx::query(
        r#"SELECT
            local_date,
            active_seconds,
            idle_seconds,
            break_seconds,
            break_count,
            eye_break_count,
            short_break_count,
            long_break_count,
            skipped_break_count,
            snoozed_reminder_count,
            longest_active_streak_seconds,
            desk_habit_score
        FROM daily_statistics
        WHERE local_date >= ?1 AND local_date <= ?2
        ORDER BY local_date ASC"#
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;

    let stats = rows
        .into_iter()
        .map(|r| TodayStatistics {
            local_date: r.get("local_date"),
            active_seconds: r.get("active_seconds"),
            idle_seconds: r.get("idle_seconds"),
            break_seconds: r.get("break_seconds"),
            break_count: r.get("break_count"),
            eye_break_count: r.get("eye_break_count"),
            short_break_count: r.get("short_break_count"),
            long_break_count: r.get("long_break_count"),
            skipped_break_count: r.get("skipped_break_count"),
            snoozed_reminder_count: r.get("snoozed_reminder_count"),
            longest_active_streak_seconds: r.get("longest_active_streak_seconds"),
            desk_habit_score: r.get("desk_habit_score"),
        })
        .collect();

    Ok(stats)
}

pub async fn increment_daily_statistics(
    pool: &SqlitePool,
    active_add: i64,
    idle_add: i64,
    break_add: i64,
) -> Result<(), AppError> {
    let local_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        r#"INSERT INTO daily_statistics (local_date, timezone_name, active_seconds, idle_seconds, break_seconds, updated_at_utc)
           VALUES (?1, '', ?2, ?3, ?4, ?5)
           ON CONFLICT(local_date) DO UPDATE SET
               active_seconds = active_seconds + ?2,
               idle_seconds = idle_seconds + ?3,
               break_seconds = break_seconds + ?4,
               updated_at_utc = ?5"#
    )
    .bind(&local_date)
    .bind(active_add)
    .bind(idle_add)
    .bind(break_add)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn increment_daily_break_stats(
    pool: &SqlitePool,
    break_type: &str,
    action: &str, // "completed", "skipped", "snoozed"
) -> Result<(), AppError> {
    let local_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Default increments
    let mut break_inc = 0;
    let mut eye_inc = 0;
    let mut short_inc = 0;
    let mut long_inc = 0;
    let mut skip_inc = 0;
    let mut snooze_inc = 0;
    
    match action {
        "completed" => {
            break_inc = 1;
            match break_type {
                "eye" => eye_inc = 1,
                "short" => short_inc = 1,
                "long" => long_inc = 1,
                _ => {}
            }
        },
        "skipped" | "dismissed" => skip_inc = 1,
        "snoozed" => snooze_inc = 1,
        _ => {}
    }
    
    sqlx::query(
        r#"INSERT INTO daily_statistics (local_date, timezone_name, updated_at_utc)
           VALUES (?1, '', ?8)
           ON CONFLICT(local_date) DO UPDATE SET
               break_count = break_count + ?2,
               eye_break_count = eye_break_count + ?3,
               short_break_count = short_break_count + ?4,
               long_break_count = long_break_count + ?5,
               skipped_break_count = skipped_break_count + ?6,
               snoozed_reminder_count = snoozed_reminder_count + ?7,
               updated_at_utc = ?8"#
    )
    .bind(&local_date)
    .bind(break_inc)
    .bind(eye_inc)
    .bind(short_inc)
    .bind(long_inc)
    .bind(skip_inc)
    .bind(snooze_inc)
    .bind(&now)
    .execute(pool)
    .await?;

    // Phase 10: Adaptive Habit Score
    // Formula: 100 - (skipped * 5) - (snoozed * 2). Min 0, Max 100.
    sqlx::query(
        r#"UPDATE daily_statistics
           SET desk_habit_score = MAX(0, MIN(100, 100 - (skipped_break_count * 5) - (snoozed_reminder_count * 2)))
           WHERE local_date = ?1"#
    )
    .bind(&local_date)
    .execute(pool)
    .await?;

    Ok(())
}

// ─── Work Sessions ─────────────────────────────────────────────────────────────

pub async fn get_recent_sessions(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<WorkSession>, AppError> {
    let rows = sqlx::query(
        r#"SELECT id, started_at_utc, ended_at_utc, active_seconds, idle_seconds, status, end_reason
        FROM work_sessions
        ORDER BY started_at_utc DESC
        LIMIT ?1"#
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let sessions = rows
        .into_iter()
        .filter_map(|r| {
            let started_str: String = r.get("started_at_utc");
            let ended_str: Option<String> = r.get("ended_at_utc");
            let status_str: String = r.get("status");

            let started = chrono::DateTime::parse_from_rfc3339(&started_str)
                .ok()?
                .with_timezone(&Utc);
            let ended = ended_str.as_deref().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|d| d.with_timezone(&Utc))
            });
            let status = match status_str.as_str() {
                "running"     => WorkSessionStatus::Running,
                "completed"   => WorkSessionStatus::Completed,
                _             => WorkSessionStatus::Interrupted,
            };
            Some(WorkSession {
                id: r.get("id"),
                started_at_utc: started,
                ended_at_utc: ended,
                active_seconds: r.get("active_seconds"),
                idle_seconds: r.get("idle_seconds"),
                status,
                end_reason: r.get("end_reason"),
            })
        })
        .collect();

    Ok(sessions)
}

/// Mark any stale 'running' sessions as interrupted (crash recovery).
pub async fn close_stale_sessions(pool: &SqlitePool) -> Result<u64, AppError> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query(
        r#"UPDATE work_sessions
           SET status = 'interrupted',
               end_reason = 'app_restart',
               ended_at_utc = ?1,
               updated_at_utc = ?1
           WHERE status = 'running'"#
    )
    .bind(now)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

// ─── Activity Segments (Phase 2) ───────────────────────────────────────────────

#[derive(sqlx::FromRow)]
pub struct SegmentRow {
    pub id: i64,
    pub state: String,
    pub ended_at_utc: Option<String>,
    pub app_identifier: Option<String>,
}

pub async fn close_stale_segments(pool: &SqlitePool) -> Result<u64, AppError> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query(
        r#"UPDATE activity_segments
           SET ended_at_utc = ?1
           WHERE ended_at_utc IS NULL"#
    )
    .bind(now)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn get_latest_segment(pool: &SqlitePool) -> Result<Option<SegmentRow>, AppError> {
    let row = sqlx::query_as::<_, SegmentRow>(
        r#"SELECT id, state, ended_at_utc, app_identifier 
           FROM activity_segments 
           ORDER BY started_at_utc DESC 
           LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn extend_segment(
    pool: &SqlitePool,
    id: i64,
    _now_str: String,
    duration_add: i64,
) -> Result<(), AppError> {
    // For open segments, we don't set ended_at_utc yet, just increase duration.
    // Wait, the prompt says "ended_at_utc TEXT" and it's open if it's NULL? 
    // Actually, maybe we just increment duration_seconds.
    sqlx::query(
        "UPDATE activity_segments SET duration_seconds = duration_seconds + ?1 WHERE id = ?2"
    )
    .bind(duration_add)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn close_segment(
    pool: &SqlitePool,
    id: i64,
    ended_at_utc: String,
) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE activity_segments SET ended_at_utc = ?1 WHERE id = ?2"
    )
    .bind(ended_at_utc)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_segment(
    pool: &SqlitePool,
    state: &str,
    started_at_utc: String,
    app_name: Option<String>,
    app_identifier: Option<String>,
) -> Result<(), AppError> {
    let created = Utc::now().to_rfc3339();
    sqlx::query(
        r#"INSERT INTO activity_segments (state, started_at_utc, duration_seconds, app_name, app_identifier, created_at_utc)
           VALUES (?1, ?2, 0, ?3, ?4, ?5)"#
    )
    .bind(state)
    .bind(started_at_utc)
    .bind(app_name)
    .bind(app_identifier)
    .bind(created)
    .execute(pool)
    .await?;
    Ok(())
}

// ─── Active Work Session Management (Phase 2) ──────────────────────────────────

#[derive(sqlx::FromRow)]
pub struct ActiveSessionRow {
    pub id: i64,
}

pub async fn get_active_work_session(pool: &SqlitePool) -> Result<Option<ActiveSessionRow>, AppError> {
    let row = sqlx::query_as::<_, ActiveSessionRow>(
        r#"SELECT id FROM work_sessions WHERE status = 'running' ORDER BY started_at_utc DESC LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert_work_session(
    pool: &SqlitePool,
    started_at_utc: String,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        r#"INSERT INTO work_sessions (started_at_utc, status, created_at_utc, updated_at_utc)
           VALUES (?1, 'running', ?2, ?2)"#
    )
    .bind(started_at_utc)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn extend_work_session(
    pool: &SqlitePool,
    id: i64,
    updated_at_utc: String,
    active_add: i64,
    idle_add: i64,
) -> Result<(), AppError> {
    sqlx::query(
        r#"UPDATE work_sessions 
           SET active_seconds = active_seconds + ?1,
               idle_seconds = idle_seconds + ?2,
               updated_at_utc = ?3
           WHERE id = ?4"#
    )
    .bind(active_add)
    .bind(idle_add)
    .bind(updated_at_utc)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn close_work_session(
    pool: &SqlitePool,
    id: i64,
    ended_at_utc: String,
    reason: &str,
) -> Result<(), AppError> {
    sqlx::query(
        r#"UPDATE work_sessions 
           SET status = 'interrupted',
               end_reason = ?1,
               ended_at_utc = ?2,
               updated_at_utc = ?2
           WHERE id = ?3"#
    )
    .bind(reason)
    .bind(ended_at_utc)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn clear_all_logs(pool: &SqlitePool) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM activity_segments").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM break_sessions").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM reminder_events").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM work_sessions").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM daily_statistics").execute(&mut *tx).await?;
    tx.commit().await?;

    let _ = sqlx::query("VACUUM").execute(pool).await;
    tracing::info!("All activity logs, sessions, reminders, and daily statistics successfully cleared");
    Ok(())
}


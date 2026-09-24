// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Settings Service
// ────────────────────────────────────────────────────────────────────────────
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::infrastructure::database::repositories;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub monitor_poll_interval_seconds: i64,
    pub activity_idle_cutoff_seconds: i64,
    pub break_eye_after_minutes: i64,
    pub break_short_after_minutes: i64,
    pub break_long_after_minutes: i64,
    pub break_eye_duration_seconds: i64,
    pub break_short_duration_seconds: i64,
    pub break_long_duration_seconds: i64,
    pub break_auto_complete_idle_seconds: i64,
    pub notification_enabled: bool,
    pub autostart_enabled: bool,
    pub ui_start_minimized: bool,
    pub privacy_track_foreground_app: bool,
    pub ai_enabled: bool,
    pub ai_endpoint: String,
    pub ai_model: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            monitor_poll_interval_seconds: 5,
            activity_idle_cutoff_seconds: 60,
            break_eye_after_minutes: 20,
            break_short_after_minutes: 45,
            break_long_after_minutes: 90,
            break_eye_duration_seconds: 20,
            break_short_duration_seconds: 180,
            break_long_duration_seconds: 300,
            break_auto_complete_idle_seconds: 180,
            notification_enabled: true,
            autostart_enabled: false,
            ui_start_minimized: false,
            privacy_track_foreground_app: false,
            ai_enabled: false,
            ai_endpoint: "http://localhost:11434/api/generate".to_string(),
            ai_model: "llama3".to_string(),
        }
    }
}

pub struct SettingsService;

impl SettingsService {
    pub async fn get_settings(pool: &SqlitePool) -> Result<AppSettings, AppError> {
        let rows = repositories::get_all_settings(pool).await?;
        let mut s = AppSettings::default();

        for row in rows {
            match row.key.as_str() {
                "monitor.poll_interval_seconds"    => s.monitor_poll_interval_seconds    = parse_i64(&row.value),
                "activity.idle_cutoff_seconds"     => s.activity_idle_cutoff_seconds     = parse_i64(&row.value),
                "break.eye_after_minutes"          => s.break_eye_after_minutes          = parse_i64(&row.value),
                "break.short_after_minutes"        => s.break_short_after_minutes        = parse_i64(&row.value),
                "break.long_after_minutes"         => s.break_long_after_minutes         = parse_i64(&row.value),
                "break.eye_duration_seconds"       => s.break_eye_duration_seconds       = parse_i64(&row.value),
                "break.short_duration_seconds"     => s.break_short_duration_seconds     = parse_i64(&row.value),
                "break.long_duration_seconds"      => s.break_long_duration_seconds      = parse_i64(&row.value),
                "break.auto_complete_idle_seconds" => s.break_auto_complete_idle_seconds = parse_i64(&row.value),
                "notification.enabled"             => s.notification_enabled             = parse_bool(&row.value),
                "autostart.enabled"                => s.autostart_enabled                = parse_bool(&row.value),
                "ui.start_minimized"               => s.ui_start_minimized               = parse_bool(&row.value),
                "privacy.track_foreground_app"     => s.privacy_track_foreground_app     = parse_bool(&row.value),
                "ai.enabled"                       => s.ai_enabled                       = parse_bool(&row.value),
                "ai.endpoint"                      => s.ai_endpoint                      = row.value.clone(),
                "ai.model"                         => s.ai_model                         = row.value.clone(),
                _ => {}
            }
        }

        Ok(s)
    }

    pub async fn update_setting(pool: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
        // Validate key is known
        let known = [
            "monitor.poll_interval_seconds",
            "activity.idle_cutoff_seconds",
            "break.eye_after_minutes",
            "break.short_after_minutes",
            "break.long_after_minutes",
            "break.eye_duration_seconds",
            "break.short_duration_seconds",
            "break.long_duration_seconds",
            "break.auto_complete_idle_seconds",
            "notification.enabled",
            "autostart.enabled",
            "ui.start_minimized",
            "privacy.track_foreground_app",
            "ai.enabled",
            "ai.endpoint",
            "ai.model",
        ];

        if !known.contains(&key) {
            return Err(AppError::Settings(format!("Unknown setting key: {key}")));
        }

        repositories::upsert_setting(pool, key, value).await
    }
}

fn parse_i64(v: &str) -> i64 {
    v.parse::<i64>().unwrap_or(0)
}

fn parse_bool(v: &str) -> bool {
    matches!(v.to_lowercase().as_str(), "true" | "1" | "yes")
}

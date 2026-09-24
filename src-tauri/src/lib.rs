// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Library Root (lib.rs)
// All modules declared here; main.rs just calls run().
// ────────────────────────────────────────────────────────────────────────────
pub mod application;
pub mod commands;
pub mod domain;
pub mod errors;
pub mod infrastructure;
pub mod scheduler;
pub mod state;



use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::{Manager, RunEvent};
use tracing_subscriber::EnvFilter;

use crate::application::activity_service::ActivityService;
use crate::application::settings_service::SettingsService;
use crate::domain::break_rules::BreakConfig;
use crate::errors::AppError;
use crate::infrastructure::database::migrations;
use crate::infrastructure::platform::detect_platform;
use crate::scheduler::monitor::run_monitor;
use crate::state::AppState;

/// Application entry point called from main.rs.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ── Logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,devbreak_lib=debug")),
        )
        .with_target(false)
        .compact()
        .init();

    tracing::info!("DevBreak starting…");

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::activity::get_current_activity,
            commands::activity::get_platform_capabilities,
            commands::activity::pause_monitoring,
            commands::activity::resume_monitoring,
            commands::activity::start_break,
            commands::activity::complete_break,
            commands::activity::snooze_reminder,
            commands::activity::dismiss_reminder,
            commands::settings::get_settings,
            commands::settings::update_setting,
            commands::settings::reset_all_data,
            commands::statistics::get_today_statistics,
            commands::statistics::get_recent_sessions,
            commands::statistics::get_statistics_range,
            commands::ai::generate_daily_tip,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // ── Initialize async runtime for setup
            tauri::async_runtime::block_on(async move {
                setup(app_handle).await
            })?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building Tauri application")
        .run(|_app, event| {
            if let RunEvent::ExitRequested { .. } = event {
                tracing::info!("DevBreak exiting");
            }
        });
}

/// Async setup: database, migrations, platform detection, monitor spawn.
async fn setup(app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // ── Database path
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Could not resolve app data dir");
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("devbreak.db");
    tracing::info!("Database path: {}", db_path.display());

    // ── Connect to SQLite
    let connect_opts = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .pragma("journal_mode", "WAL")
        .pragma("foreign_keys", "ON")
        .pragma("busy_timeout", "5000");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .map_err(|e| AppError::Database(e))?;

    // ── Run migrations
    migrations::run_migrations(&pool).await?;

    // ── Crash recovery: close stale sessions and segments
    ActivityService::recover_crash_state(&pool).await?;

    // ── Load settings
    let settings = SettingsService::get_settings(&pool).await?;
    let poll_interval = settings.monitor_poll_interval_seconds.max(1) as u64;
    let idle_cutoff = settings.activity_idle_cutoff_seconds.max(5) as u64;
    let break_config = BreakConfig {
        eye_after_seconds:          (settings.break_eye_after_minutes   * 60) as u64,
        short_after_seconds:        (settings.break_short_after_minutes  * 60) as u64,
        long_after_seconds:         (settings.break_long_after_minutes   * 60) as u64,
        eye_duration_seconds:        settings.break_eye_duration_seconds       as u64,
        short_duration_seconds:      settings.break_short_duration_seconds     as u64,
        long_duration_seconds:       settings.break_long_duration_seconds      as u64,
        auto_complete_idle_seconds:  settings.break_auto_complete_idle_seconds as u64,
    };

    // ── Detect platform
    let platform = detect_platform().await;
    let capabilities = platform.capabilities();
    tracing::info!("Platform capabilities: {:?}", capabilities);

    // ── Build app state
    let app_state = AppState::new(pool, capabilities, platform.clone());
    let current_activity = app_state.current_activity.clone();
    let monitoring_paused = app_state.monitoring_paused.clone();

    // ── Register managed state
    app_handle.manage(app_state);

    // ── Spawn background monitor
    let monitor_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        run_monitor(
            monitor_handle,
            platform,
            current_activity,
            monitoring_paused,
            poll_interval,
            idle_cutoff,
            break_config,
        )
        .await;
    });

    // ── Setup Tray Icon
    crate::application::tray::setup_tray(&app_handle)?;

    tracing::info!("DevBreak setup complete");
    Ok(())
}


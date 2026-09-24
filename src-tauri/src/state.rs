// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Shared Application State
// Holds all shared state passed as Tauri managed state.
// ────────────────────────────────────────────────────────────────────────────
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::activity::{ActivityState, CurrentActivity};
use crate::infrastructure::platform::{PlatformCapabilities, PlatformProvider};

/// All shared state managed by Tauri.
pub struct AppState {
    /// SQLite connection pool
    pub db: SqlitePool,

    /// Current activity (updated by background monitor)
    pub current_activity: Arc<RwLock<CurrentActivity>>,

    /// Platform capabilities (set once on startup)
    pub capabilities: Arc<RwLock<PlatformCapabilities>>,

    /// Platform provider (idle detection, lock detection)
    pub platform: Arc<dyn PlatformProvider + Send + Sync>,

    /// Whether monitoring is paused by user
    pub monitoring_paused: Arc<RwLock<bool>>,
}

impl AppState {
    pub fn new(
        db: SqlitePool,
        capabilities: PlatformCapabilities,
        platform: Arc<dyn PlatformProvider + Send + Sync>,
    ) -> Self {
        let current_activity = CurrentActivity {
            state: ActivityState::Initializing,
            idle_seconds: 0,
            active_session_seconds: 0,
            next_eye_break_seconds: None,
            next_short_break_seconds: None,
            next_long_break_seconds: None,
            last_updated_utc: chrono::Utc::now(),
        };

        Self {
            db,
            current_activity: Arc::new(RwLock::new(current_activity)),
            capabilities: Arc::new(RwLock::new(capabilities)),
            platform,
            monitoring_paused: Arc::new(RwLock::new(false)),
        }
    }
}

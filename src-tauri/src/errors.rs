// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Error Types
// ────────────────────────────────────────────────────────────────────────────
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Platform error: {0}")]
    Platform(#[from] PlatformError),

    #[error("Settings error: {0}")]
    Settings(String),

    #[error("Activity error: {0}")]
    Activity(String),

    #[error("Notification error: {0}")]
    Notification(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("Idle detection unavailable: {0}")]
    IdleDetectionUnavailable(String),

    #[error("D-Bus error: {0}")]
    DBus(String),

    #[error("Screen lock detection unavailable")]
    LockDetectionUnavailable,

    #[error("Platform not supported: {0}")]
    NotSupported(String),
}

// Allow Tauri commands to serialize errors as strings
impl serde::Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

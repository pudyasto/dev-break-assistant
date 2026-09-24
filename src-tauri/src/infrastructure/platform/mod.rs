// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Platform Abstraction Layer
// All OS-specific code lives behind these traits.
// ────────────────────────────────────────────────────────────────────────────
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::errors::PlatformError;

// ─── Capabilities ─────────────────────────────────────────────────────────────

/// Describes what this platform can detect.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformCapabilities {
    pub idle_detection: bool,
    pub session_lock_detection: bool,
    pub foreground_app_detection: bool,
    pub platform: String,
    pub desktop_environment: Option<String>,
}

impl PlatformCapabilities {
    /// A minimal fallback when nothing is available.
    pub fn none() -> Self {
        Self {
            idle_detection: false,
            session_lock_detection: false,
            foreground_app_detection: false,
            platform: std::env::consts::OS.to_string(),
            desktop_environment: None,
        }
    }
}

// ─── Traits ───────────────────────────────────────────────────────────────────

/// Provides idle time detection.
#[async_trait]
pub trait IdleProvider: Send + Sync {
    /// Returns how many seconds the user has been idle.
    async fn idle_seconds(&self) -> Result<u64, PlatformError>;
}

/// Provides screen lock / session state detection.
#[async_trait]
pub trait SessionProvider: Send + Sync {
    /// Returns true if the screen is locked.
    async fn is_locked(&self) -> Result<bool, PlatformError>;

    /// Returns true if the system is suspended.
    async fn is_suspended(&self) -> Result<bool, PlatformError>;
}

pub struct ForegroundApp {
    pub name: String,
    pub identifier: Option<String>,
}

/// A combined platform provider (idle + session + foreground).
#[async_trait]
pub trait PlatformProvider: Send + Sync {
    async fn idle_seconds(&self) -> Result<u64, PlatformError>;
    async fn is_locked(&self) -> Result<bool, PlatformError>;
    async fn is_suspended(&self) -> Result<bool, PlatformError>;
    
    /// Phase 8: Optional foreground application detection.
    async fn foreground_app(&self) -> Result<Option<ForegroundApp>, PlatformError> {
        Ok(None) // Default to None if unsupported
    }
    
    fn capabilities(&self) -> PlatformCapabilities;
}

// ─── Fallback (no-op) implementation ──────────────────────────────────────────

/// Used when no platform-specific provider is available.
/// Always returns 0 idle seconds (user is always "active").
pub struct FallbackProvider;

#[async_trait]
impl PlatformProvider for FallbackProvider {
    async fn idle_seconds(&self) -> Result<u64, PlatformError> {
        // Fallback: we cannot detect idle time.
        // Return 0 so the user is always considered active.
        Ok(0)
    }

    async fn is_locked(&self) -> Result<bool, PlatformError> {
        Ok(false)
    }

    async fn is_suspended(&self) -> Result<bool, PlatformError> {
        Ok(false)
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities::none()
    }
}

// ─── Platform detection ───────────────────────────────────────────────────────

/// Detects the current platform and returns the best available provider.
pub async fn detect_platform() -> Arc<dyn PlatformProvider + Send + Sync> {
    #[cfg(target_os = "linux")]
    {
        tracing::info!("Platform: Linux — attempting GNOME Mutter D-Bus idle provider");
        match linux::GnomeMutterProvider::new().await {
            Ok(provider) => {
                tracing::info!("GNOME Mutter idle provider initialized successfully");
                return Arc::new(provider);
            }
            Err(e) => {
                tracing::warn!("GNOME Mutter provider unavailable: {e}. Using fallback.");
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        tracing::info!("Platform: macOS — using CoreGraphics idle provider");
        return Arc::new(macos::MacOsProvider::new());
    }

    tracing::warn!("No platform-specific idle provider available. Using fallback.");
    Arc::new(FallbackProvider)
}

// ─── Submodules ───────────────────────────────────────────────────────────────

pub mod linux;
pub mod macos;

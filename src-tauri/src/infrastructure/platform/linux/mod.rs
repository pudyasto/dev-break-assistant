// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Linux Platform Provider
// Uses org.gnome.Mutter.IdleMonitor via D-Bus (zbus) for idle detection.
// Gracefully degrades when GNOME / Mutter is not available.
// ────────────────────────────────────────────────────────────────────────────

#![cfg(target_os = "linux")]

use async_trait::async_trait;
use zbus::{proxy, Connection};

use super::{PlatformCapabilities, PlatformProvider};
use crate::errors::PlatformError;

// ─── D-Bus Proxy ──────────────────────────────────────────────────────────────

#[proxy(
    interface = "org.gnome.Mutter.IdleMonitor",
    default_service = "org.gnome.Mutter.IdleMonitor",
    default_path = "/org/gnome/Mutter/IdleMonitor/Core"
)]
trait MutterIdleMonitor {
    /// Returns idle time in milliseconds.
    async fn get_idletime(&self) -> zbus::Result<u64>;
}

// ─── Provider ─────────────────────────────────────────────────────────────────

pub struct GnomeMutterProvider {
    connection: Connection,
    desktop_environment: String,
}

impl GnomeMutterProvider {
    /// Attempt to connect to GNOME Mutter D-Bus interface.
    /// Returns Err if the interface is unavailable.
    pub async fn new() -> Result<Self, PlatformError> {
        let connection = Connection::session()
            .await
            .map_err(|e| PlatformError::DBus(format!("Cannot connect to D-Bus session: {e}")))?;

        // Detect desktop environment
        let desktop_environment = detect_desktop_environment();
        tracing::debug!("Desktop environment: {desktop_environment}");

        // Verify the Mutter interface is accessible
        let proxy = MutterIdleMonitorProxy::new(&connection)
            .await
            .map_err(|e| PlatformError::DBus(format!("Cannot create Mutter proxy: {e}")))?;

        proxy
            .get_idletime()
            .await
            .map_err(|e| PlatformError::DBus(format!("Mutter.GetIdletime call failed: {e}")))?;

        tracing::info!("GNOME Mutter IdleMonitor D-Bus interface available");

        Ok(Self {
            connection,
            desktop_environment,
        })
    }

    async fn get_idle_ms(&self) -> Result<u64, PlatformError> {
        let proxy = MutterIdleMonitorProxy::new(&self.connection)
            .await
            .map_err(|e| PlatformError::DBus(format!("Proxy error: {e}")))?;

        proxy
            .get_idletime()
            .await
            .map_err(|e| PlatformError::DBus(format!("GetIdletime error: {e}")))
    }
}

#[async_trait]
impl PlatformProvider for GnomeMutterProvider {
    async fn idle_seconds(&self) -> Result<u64, PlatformError> {
        let ms = self.get_idle_ms().await?;
        Ok(ms / 1000)
    }

    async fn is_locked(&self) -> Result<bool, PlatformError> {
        // Screen lock detection via GNOME ScreenSaver D-Bus
        match is_screen_locked(&self.connection).await {
            Ok(locked) => Ok(locked),
            Err(e) => {
                tracing::trace!("Screen lock detection failed (non-fatal): {e}");
                Ok(false)
            }
        }
    }

    async fn is_suspended(&self) -> Result<bool, PlatformError> {
        // Suspension is handled by monitoring idle increase; not directly detectable
        Ok(false)
    }

    async fn foreground_app(&self) -> Result<Option<super::ForegroundApp>, PlatformError> {
        // Best effort via xdotool (works on X11 and XWayland).
        // For native GNOME Wayland, this requires an extension or specialized D-Bus.
        use std::process::Command;
        
        let output = Command::new("xdotool")
            .args(&["getactivewindow", "getwindowclassname"])
            .output();
            
        match output {
            Ok(out) if out.status.success() => {
                let identifier = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !identifier.is_empty() {
                    return Ok(Some(super::ForegroundApp {
                        name: identifier.clone(), // Basic fallback, often same as class name
                        identifier: Some(identifier),
                    }));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            idle_detection: true,
            session_lock_detection: true,
            foreground_app_detection: true, // We do best-effort via xdotool
            platform: "linux".to_string(),
            desktop_environment: Some(self.desktop_environment.clone()),
        }
    }
}

// ─── Screen lock detection ────────────────────────────────────────────────────

#[proxy(
    interface = "org.gnome.ScreenSaver",
    default_service = "org.gnome.ScreenSaver",
    default_path = "/org/gnome/ScreenSaver"
)]
trait GnomeScreenSaver {
    async fn get_active(&self) -> zbus::Result<bool>;
}

async fn is_screen_locked(connection: &Connection) -> Result<bool, PlatformError> {
    let proxy = GnomeScreenSaverProxy::new(connection)
        .await
        .map_err(|e| PlatformError::DBus(e.to_string()))?;

    proxy
        .get_active()
        .await
        .map_err(|e| PlatformError::DBus(e.to_string()))
}

// ─── Desktop environment detection ───────────────────────────────────────────

fn detect_desktop_environment() -> String {
    // Check environment variables in priority order
    for var in &["DESKTOP_SESSION", "XDG_CURRENT_DESKTOP", "XDG_SESSION_DESKTOP"] {
        if let Ok(val) = std::env::var(var) {
            if !val.is_empty() {
                return val;
            }
        }
    }
    "Unknown".to_string()
}

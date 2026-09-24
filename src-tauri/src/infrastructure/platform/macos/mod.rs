// ────────────────────────────────────────────────────────────────────────────
// DevBreak — macOS Platform Provider (Phase 7)
// Uses CoreGraphics CGEventSourceSecondsSinceLastEventType for idle detection
// ────────────────────────────────────────────────────────────────────────────

use async_trait::async_trait;
use super::{PlatformCapabilities, PlatformProvider};
use crate::errors::PlatformError;

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventSourceSecondsSinceLastEventType(
        sourceStateID: i32,
        eventType: u32,
    ) -> f64;
}

#[cfg(target_os = "macos")]
const K_CG_EVENT_SOURCE_STATE_HID_SYSTEM_STATE: i32 = 1;
#[cfg(target_os = "macos")]
const K_CG_ANY_INPUT_EVENT_TYPE: u32 = 4294967295; // u32::MAX

pub struct MacOsProvider;

impl MacOsProvider {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl PlatformProvider for MacOsProvider {
    async fn idle_seconds(&self) -> Result<u64, PlatformError> {
        #[cfg(target_os = "macos")]
        {
            let seconds = unsafe {
                CGEventSourceSecondsSinceLastEventType(
                    K_CG_EVENT_SOURCE_STATE_HID_SYSTEM_STATE,
                    K_CG_ANY_INPUT_EVENT_TYPE,
                )
            };
            Ok(seconds as u64)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(PlatformError::NotSupported("macOS idle provider only available on macOS".into()))
        }
    }

    async fn is_locked(&self) -> Result<bool, PlatformError> {
        // Advanced lock detection requires CoreFoundation / Quartz.
        // For now, we return false as a basic fallback.
        Ok(false)
    }

    async fn is_suspended(&self) -> Result<bool, PlatformError> {
        Ok(false)
    }

    async fn foreground_app(&self) -> Result<Option<super::ForegroundApp>, PlatformError> {
        use std::process::Command;
        let output = Command::new("osascript")
            .args(&["-e", "tell application \"System Events\" to get name of first application process whose frontmost is true"])
            .output();
            
        match output {
            Ok(out) if out.status.success() => {
                let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !name.is_empty() {
                    return Ok(Some(super::ForegroundApp {
                        identifier: Some(name.clone()),
                        name,
                    }));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            idle_detection: true, // we implemented this
            session_lock_detection: false,
            foreground_app_detection: true,
            platform: "macos".to_string(),
            desktop_environment: Some("macOS".to_string()),
        }
    }
}

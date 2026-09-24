// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Domain: Activity State Machine
// ────────────────────────────────────────────────────────────────────────────
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The canonical activity states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActivityState {
    Initializing,
    Active,
    Idle,
    BreakDue,
    Breaking,
    Locked,
    Suspended,
    Paused,
}

impl std::fmt::Display for ActivityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityState::Initializing => write!(f, "Initializing"),
            ActivityState::Active       => write!(f, "Active"),
            ActivityState::Idle         => write!(f, "Idle"),
            ActivityState::BreakDue     => write!(f, "BreakDue"),
            ActivityState::Breaking     => write!(f, "Breaking"),
            ActivityState::Locked       => write!(f, "Locked"),
            ActivityState::Suspended    => write!(f, "Suspended"),
            ActivityState::Paused       => write!(f, "Paused"),
        }
    }
}

/// Snapshot of current activity, sent to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentActivity {
    pub state: ActivityState,
    pub idle_seconds: u64,
    pub active_session_seconds: u64,
    pub next_eye_break_seconds: Option<u64>,
    pub next_short_break_seconds: Option<u64>,
    pub next_long_break_seconds: Option<u64>,
    pub last_updated_utc: DateTime<Utc>,
}

/// Determines the activity state from idle seconds and cutoff.
pub fn evaluate_state(idle_seconds: u64, idle_cutoff: u64, current: &ActivityState) -> ActivityState {
    // Don't override locked/suspended/paused/breaking from idle alone
    match current {
        ActivityState::Locked | ActivityState::Suspended | ActivityState::Paused => {
            return current.clone();
        }
        ActivityState::Breaking => {
            // Stay in breaking until sufficient idle accumulates (handled by break engine)
            return current.clone();
        }
        _ => {}
    }

    if idle_seconds >= idle_cutoff {
        ActivityState::Idle
    } else {
        ActivityState::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_when_idle_below_cutoff() {
        let state = evaluate_state(30, 60, &ActivityState::Active);
        assert_eq!(state, ActivityState::Active);
    }

    #[test]
    fn idle_when_idle_above_cutoff() {
        let state = evaluate_state(90, 60, &ActivityState::Active);
        assert_eq!(state, ActivityState::Idle);
    }

    #[test]
    fn locked_persists() {
        let state = evaluate_state(0, 60, &ActivityState::Locked);
        assert_eq!(state, ActivityState::Locked);
    }
}

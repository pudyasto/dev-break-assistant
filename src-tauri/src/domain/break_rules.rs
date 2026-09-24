// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Domain: Break Rules
// Determines when breaks are due based on active session accumulation.
// ────────────────────────────────────────────────────────────────────────────
use serde::{Deserialize, Serialize};

/// Break rule configuration (mirrors app_settings).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakConfig {
    pub eye_after_seconds: u64,
    pub short_after_seconds: u64,
    pub long_after_seconds: u64,
    pub eye_duration_seconds: u64,
    pub short_duration_seconds: u64,
    pub long_duration_seconds: u64,
    pub auto_complete_idle_seconds: u64,
}

impl Default for BreakConfig {
    fn default() -> Self {
        Self {
            eye_after_seconds: 20 * 60,
            short_after_seconds: 45 * 60,
            long_after_seconds: 90 * 60,
            eye_duration_seconds: 20,
            short_duration_seconds: 180,
            long_duration_seconds: 300,
            auto_complete_idle_seconds: 180,
        }
    }
}

/// What break (if any) is currently due.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakDue {
    None,
    Eye,
    Short,
    Long,
}

/// Evaluate which break is due given active session seconds.
/// Priority: Long > Short > Eye.
pub fn evaluate_break_due(active_seconds: u64, config: &BreakConfig) -> BreakDue {
    if active_seconds >= config.long_after_seconds {
        return BreakDue::Long;
    }
    if active_seconds >= config.short_after_seconds {
        return BreakDue::Short;
    }
    if active_seconds >= config.eye_after_seconds {
        return BreakDue::Eye;
    }
    BreakDue::None
}

/// Seconds until next break of each type.
pub struct BreakCountdowns {
    pub eye: Option<u64>,
    pub short: Option<u64>,
    pub long: Option<u64>,
}

pub fn compute_countdowns(active_seconds: u64, config: &BreakConfig) -> BreakCountdowns {
    let eye = if active_seconds < config.eye_after_seconds {
        Some(config.eye_after_seconds - active_seconds)
    } else {
        Some(0)
    };
    let short = if active_seconds < config.short_after_seconds {
        Some(config.short_after_seconds - active_seconds)
    } else {
        Some(0)
    };
    let long = if active_seconds < config.long_after_seconds {
        Some(config.long_after_seconds - active_seconds)
    } else {
        Some(0)
    };

    BreakCountdowns { eye, short, long }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> BreakConfig { BreakConfig::default() }

    #[test]
    fn no_break_at_start() {
        assert_eq!(evaluate_break_due(0, &cfg()), BreakDue::None);
    }

    #[test]
    fn eye_break_at_20min() {
        assert_eq!(evaluate_break_due(20 * 60, &cfg()), BreakDue::Eye);
    }

    #[test]
    fn short_break_at_45min() {
        assert_eq!(evaluate_break_due(45 * 60, &cfg()), BreakDue::Short);
    }

    #[test]
    fn long_break_at_90min() {
        assert_eq!(evaluate_break_due(90 * 60, &cfg()), BreakDue::Long);
    }
}

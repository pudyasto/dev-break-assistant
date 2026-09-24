// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Domain: Statistics types
// ────────────────────────────────────────────────────────────────────────────
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TodayStatistics {
    pub local_date: String,
    pub active_seconds: i64,
    pub idle_seconds: i64,
    pub break_seconds: i64,
    pub break_count: i64,
    pub eye_break_count: i64,
    pub short_break_count: i64,
    pub long_break_count: i64,
    pub skipped_break_count: i64,
    pub snoozed_reminder_count: i64,
    pub longest_active_streak_seconds: i64,
    pub desk_habit_score: Option<i64>,
}

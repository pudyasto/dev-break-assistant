// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Domain: Session types
// ────────────────────────────────────────────────────────────────────────────
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkSession {
    pub id: i64,
    pub started_at_utc: DateTime<Utc>,
    pub ended_at_utc: Option<DateTime<Utc>>,
    pub active_seconds: i64,
    pub idle_seconds: i64,
    pub status: WorkSessionStatus,
    pub end_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkSessionStatus {
    Running,
    Completed,
    Interrupted,
}

impl std::fmt::Display for WorkSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkSessionStatus::Running     => write!(f, "running"),
            WorkSessionStatus::Completed   => write!(f, "completed"),
            WorkSessionStatus::Interrupted => write!(f, "interrupted"),
        }
    }
}

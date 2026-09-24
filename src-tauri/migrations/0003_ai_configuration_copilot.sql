-- Auditable, reversible configuration changes made through the AI copilot.
CREATE TABLE IF NOT EXISTS ai_configuration_changes (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    changes_before_json TEXT NOT NULL,
    changes_after_json  TEXT NOT NULL,
    reason             TEXT NOT NULL,
    model              TEXT NOT NULL,
    created_at_utc     TEXT NOT NULL,
    undone_at_utc      TEXT
);

CREATE INDEX IF NOT EXISTS idx_ai_configuration_changes_created
    ON ai_configuration_changes(created_at_utc DESC);

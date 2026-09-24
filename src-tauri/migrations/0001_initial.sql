-- DevBreak Initial Migration
-- Phase 0: All core tables for Phase 0–5

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA busy_timeout = 5000;

-- ──────────────────────────────────────────────
-- 1. app_settings
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS app_settings (
    key          TEXT PRIMARY KEY,
    value        TEXT    NOT NULL,
    value_type   TEXT    NOT NULL DEFAULT 'string',
    updated_at_utc TEXT NOT NULL
);

-- Seed default settings
INSERT OR IGNORE INTO app_settings (key, value, value_type, updated_at_utc) VALUES
    ('monitor.poll_interval_seconds',    '5',     'integer', datetime('now')),
    ('activity.idle_cutoff_seconds',     '60',    'integer', datetime('now')),
    ('break.eye_after_minutes',          '20',    'integer', datetime('now')),
    ('break.short_after_minutes',        '45',    'integer', datetime('now')),
    ('break.long_after_minutes',         '90',    'integer', datetime('now')),
    ('break.eye_duration_seconds',       '20',    'integer', datetime('now')),
    ('break.short_duration_seconds',     '180',   'integer', datetime('now')),
    ('break.long_duration_seconds',      '300',   'integer', datetime('now')),
    ('break.auto_complete_idle_seconds', '180',   'integer', datetime('now')),
    ('notification.enabled',             'true',  'boolean', datetime('now')),
    ('autostart.enabled',                'false', 'boolean', datetime('now')),
    ('ui.start_minimized',               'false', 'boolean', datetime('now')),
    ('privacy.track_foreground_app',     'false', 'boolean', datetime('now'));

-- ──────────────────────────────────────────────
-- 2. activity_segments
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS activity_segments (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    state           TEXT NOT NULL CHECK (state IN ('active', 'idle', 'locked', 'suspended')),
    started_at_utc  TEXT NOT NULL,
    ended_at_utc    TEXT,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    source          TEXT,
    app_name        TEXT,      -- NULL unless foreground tracking enabled
    app_identifier  TEXT,      -- NULL unless foreground tracking enabled
    created_at_utc  TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_activity_segments_started
    ON activity_segments(started_at_utc);

CREATE INDEX IF NOT EXISTS idx_activity_segments_state
    ON activity_segments(state);

CREATE INDEX IF NOT EXISTS idx_activity_segments_app
    ON activity_segments(app_identifier);

-- ──────────────────────────────────────────────
-- 3. work_sessions
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS work_sessions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at_utc  TEXT NOT NULL,
    ended_at_utc    TEXT,
    active_seconds  INTEGER NOT NULL DEFAULT 0,
    idle_seconds    INTEGER NOT NULL DEFAULT 0,
    status          TEXT NOT NULL CHECK (status IN ('running', 'completed', 'interrupted')),
    end_reason      TEXT CHECK (end_reason IN ('break', 'manual', 'locked', 'suspended', 'shutdown', 'app_restart')),
    created_at_utc  TEXT NOT NULL,
    updated_at_utc  TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_work_sessions_started
    ON work_sessions(started_at_utc);

CREATE INDEX IF NOT EXISTS idx_work_sessions_status
    ON work_sessions(status);

-- ──────────────────────────────────────────────
-- 4. break_sessions
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS break_sessions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    work_session_id INTEGER,
    break_type      TEXT NOT NULL CHECK (break_type IN ('eye', 'short', 'long', 'manual')),
    source          TEXT NOT NULL CHECK (source IN ('reminder', 'idle', 'manual', 'screen_lock')),
    started_at_utc  TEXT NOT NULL,
    ended_at_utc    TEXT,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    status          TEXT NOT NULL CHECK (status IN ('started', 'completed', 'skipped', 'interrupted')),
    completion_reason TEXT,
    created_at_utc  TEXT NOT NULL,
    FOREIGN KEY(work_session_id) REFERENCES work_sessions(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_break_sessions_started
    ON break_sessions(started_at_utc);

CREATE INDEX IF NOT EXISTS idx_break_sessions_type
    ON break_sessions(break_type);

-- ──────────────────────────────────────────────
-- 5. reminder_events
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reminder_events (
    id                         INTEGER PRIMARY KEY AUTOINCREMENT,
    work_session_id            INTEGER,
    break_type                 TEXT NOT NULL CHECK (break_type IN ('eye', 'short', 'long')),
    triggered_at_utc           TEXT NOT NULL,
    active_seconds_at_trigger  INTEGER NOT NULL,
    action                     TEXT CHECK (action IN ('started', 'snoozed', 'dismissed', 'timeout')),
    action_at_utc              TEXT,
    snooze_until_utc           TEXT,
    created_at_utc             TEXT NOT NULL,
    FOREIGN KEY(work_session_id) REFERENCES work_sessions(id) ON DELETE SET NULL
);

-- ──────────────────────────────────────────────
-- 6. daily_statistics
-- ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS daily_statistics (
    local_date                    TEXT PRIMARY KEY,
    timezone_name                 TEXT,
    active_seconds                INTEGER NOT NULL DEFAULT 0,
    idle_seconds                  INTEGER NOT NULL DEFAULT 0,
    break_seconds                 INTEGER NOT NULL DEFAULT 0,
    break_count                   INTEGER NOT NULL DEFAULT 0,
    eye_break_count               INTEGER NOT NULL DEFAULT 0,
    short_break_count             INTEGER NOT NULL DEFAULT 0,
    long_break_count              INTEGER NOT NULL DEFAULT 0,
    skipped_break_count           INTEGER NOT NULL DEFAULT 0,
    snoozed_reminder_count        INTEGER NOT NULL DEFAULT 0,
    longest_active_streak_seconds INTEGER NOT NULL DEFAULT 0,
    desk_habit_score              INTEGER,
    updated_at_utc                TEXT NOT NULL
);

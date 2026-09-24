// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Shared TypeScript Types
// All types mirror Rust structs serialized via Tauri commands.
// ────────────────────────────────────────────────────────────────────────────

// ─── Activity State ──────────────────────────────────────────────────────────

export type ActivityState =
  | 'Initializing'
  | 'Active'
  | 'Idle'
  | 'BreakDue'
  | 'Breaking'
  | 'Locked'
  | 'Suspended'
  | 'Paused'

// ─── Platform ────────────────────────────────────────────────────────────────

export interface PlatformCapabilities {
  idleDetection: boolean
  sessionLockDetection: boolean
  foregroundAppDetection: boolean
  platform: string
  desktopEnvironment: string | null
}

// ─── Activity ────────────────────────────────────────────────────────────────

export interface CurrentActivity {
  state: ActivityState
  idleSeconds: number
  activeSessionSeconds: number
  nextEyeBreakSeconds: number | null
  nextShortBreakSeconds: number | null
  nextLongBreakSeconds: number | null
  lastUpdatedUtc: string
}

// ─── Sessions ────────────────────────────────────────────────────────────────

export interface WorkSession {
  id: number
  startedAtUtc: string
  endedAtUtc: string | null
  activeSeconds: number
  idleSeconds: number
  status: 'running' | 'completed' | 'interrupted'
  endReason: string | null
}

export interface BreakSession {
  id: number
  workSessionId: number | null
  breakType: 'eye' | 'short' | 'long' | 'manual'
  source: 'reminder' | 'idle' | 'manual' | 'screen_lock'
  startedAtUtc: string
  endedAtUtc: string | null
  durationSeconds: number
  status: 'started' | 'completed' | 'skipped' | 'interrupted'
  completionReason: string | null
}

// ─── Statistics ───────────────────────────────────────────────────────────────

export interface TodayStatistics {
  localDate: string
  activeSeconds: number
  idleSeconds: number
  breakSeconds: number
  breakCount: number
  eyeBreakCount: number
  shortBreakCount: number
  longBreakCount: number
  skippedBreakCount: number
  snoozedReminderCount: number
  longestActiveStreakSeconds: number
  deskHabitScore: number | null
}

// ─── Settings ────────────────────────────────────────────────────────────────

export interface AppSettings {
  monitorPollIntervalSeconds: number
  activityIdleCutoffSeconds: number
  breakEyeAfterMinutes: number
  breakShortAfterMinutes: number
  breakLongAfterMinutes: number
  breakEyeDurationSeconds: number
  breakShortDurationSeconds: number
  breakLongDurationSeconds: number
  breakAutoCompleteIdleSeconds: number
  notificationEnabled: boolean
  autostartEnabled: boolean
  uiStartMinimized: boolean
  privacyTrackForegroundApp: boolean
  aiEnabled: boolean
  aiEndpoint: string
  aiModel: string
}

// ─── Events (Tauri) ──────────────────────────────────────────────────────────

export interface ActivityChangedPayload {
  state: ActivityState
  idleSeconds: number
  activeSessionSeconds: number
}

// ─── UI helpers ──────────────────────────────────────────────────────────────

export function formatDuration(seconds: number): string {
  if (seconds < 60) {
    return `${seconds}s`
  }
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  if (m < 60) {
    return s > 0 ? `${m}m ${s}s` : `${m}m`
  }
  const h = Math.floor(m / 60)
  const rem = m % 60
  return rem > 0 ? `${h}h ${rem}m` : `${h}h`
}

export function formatDurationShort(seconds: number): string {
  if (seconds < 60) return `${seconds}s`
  const m = Math.floor(seconds / 60)
  if (m < 60) return `${m}m`
  const h = Math.floor(m / 60)
  const rem = m % 60
  return rem > 0 ? `${h}h ${rem}m` : `${h}h`
}

export function stateLabel(state: ActivityState): string {
  const labels: Record<ActivityState, string> = {
    Initializing: 'Initializing',
    Active:       'Active',
    Idle:         'Idle',
    BreakDue:     'Break Due',
    Breaking:     'On Break',
    Locked:       'Screen Locked',
    Suspended:    'Suspended',
    Paused:       'Paused',
  }
  return labels[state] ?? state
}

export function stateColor(state: ActivityState): string {
  const colors: Record<ActivityState, string> = {
    Initializing: 'text-muted',
    Active:       'text-green-400',
    Idle:         'text-amber-400',
    BreakDue:     'text-red-400',
    Breaking:     'text-blue-400',
    Locked:       'text-purple-400',
    Suspended:    'text-gray-400',
    Paused:       'text-yellow-400',
  }
  return colors[state] ?? 'text-muted'
}

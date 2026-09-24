// ────────────────────────────────────────────────────────────────────────────
// DevBreak — Tauri Command Bridge
// All Rust ↔ Vue communication goes through this module.
// No Vue component should call `invoke` directly.
// ────────────────────────────────────────────────────────────────────────────

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type {
  AppSettings,
  CurrentActivity,
  PlatformCapabilities,
  TodayStatistics,
  WorkSession,
  ActivityChangedPayload,
} from '@/types'

// ─── Activity Commands ────────────────────────────────────────────────────────

export async function getCurrentActivity(): Promise<CurrentActivity> {
  return invoke<CurrentActivity>('get_current_activity')
}

export async function getPlatformCapabilities(): Promise<PlatformCapabilities> {
  return invoke<PlatformCapabilities>('get_platform_capabilities')
}

export async function pauseMonitoring(): Promise<void> {
  return invoke('pause_monitoring')
}

export async function resumeMonitoring(): Promise<void> {
  return invoke('resume_monitoring')
}

export async function startBreak(breakType: string): Promise<void> {
  return invoke('start_break', { breakType })
}

export async function completeBreak(breakType: string, durationSeconds: number): Promise<void> {
  return invoke('complete_break', { breakType, durationSeconds })
}

export async function snoozeReminder(breakType: string): Promise<void> {
  return invoke('snooze_reminder', { breakType })
}

export async function dismissReminder(breakType: string): Promise<void> {
  return invoke('dismiss_reminder', { breakType })
}

// ─── Settings Commands ────────────────────────────────────────────────────────

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings')
}

export async function updateSetting(key: string, value: string): Promise<void> {
  return invoke('update_setting', { key, value })
}

export async function updateSettings(settings: Partial<AppSettings>): Promise<void> {
  const promises: Promise<void>[] = []

  if (settings.monitorPollIntervalSeconds !== undefined)
    promises.push(updateSetting('monitor.poll_interval_seconds', String(settings.monitorPollIntervalSeconds)))
  if (settings.activityIdleCutoffSeconds !== undefined)
    promises.push(updateSetting('activity.idle_cutoff_seconds', String(settings.activityIdleCutoffSeconds)))
  if (settings.breakEyeAfterMinutes !== undefined)
    promises.push(updateSetting('break.eye_after_minutes', String(settings.breakEyeAfterMinutes)))
  if (settings.breakShortAfterMinutes !== undefined)
    promises.push(updateSetting('break.short_after_minutes', String(settings.breakShortAfterMinutes)))
  if (settings.breakLongAfterMinutes !== undefined)
    promises.push(updateSetting('break.long_after_minutes', String(settings.breakLongAfterMinutes)))
  if (settings.breakEyeDurationSeconds !== undefined)
    promises.push(updateSetting('break.eye_duration_seconds', String(settings.breakEyeDurationSeconds)))
  if (settings.breakShortDurationSeconds !== undefined)
    promises.push(updateSetting('break.short_duration_seconds', String(settings.breakShortDurationSeconds)))
  if (settings.breakLongDurationSeconds !== undefined)
    promises.push(updateSetting('break.long_duration_seconds', String(settings.breakLongDurationSeconds)))
  if (settings.breakAutoCompleteIdleSeconds !== undefined)
    promises.push(updateSetting('break.auto_complete_idle_seconds', String(settings.breakAutoCompleteIdleSeconds)))
  if (settings.notificationEnabled !== undefined)
    promises.push(updateSetting('notification.enabled', settings.notificationEnabled ? 'true' : 'false'))
  if (settings.autostartEnabled !== undefined)
    promises.push(updateSetting('autostart.enabled', settings.autostartEnabled ? 'true' : 'false'))
  if (settings.uiStartMinimized !== undefined)
    promises.push(updateSetting('ui.start_minimized', settings.uiStartMinimized ? 'true' : 'false'))
  if (settings.privacyTrackForegroundApp !== undefined)
    promises.push(updateSetting('privacy.track_foreground_app', settings.privacyTrackForegroundApp ? 'true' : 'false'))
  if (settings.aiEnabled !== undefined)
    promises.push(updateSetting('ai.enabled', settings.aiEnabled ? 'true' : 'false'))
  if (settings.aiEndpoint !== undefined)
    promises.push(updateSetting('ai.endpoint', settings.aiEndpoint))
  if (settings.aiModel !== undefined)
    promises.push(updateSetting('ai.model', settings.aiModel))

  await Promise.all(promises)
}

export async function testAiConnection(endpoint: string, model: string): Promise<string> {
  return invoke<string>('test_ai_connection', { endpoint, model })
}

export async function resetAllData(): Promise<void> {
  return invoke('reset_all_data')
}

// ─── Statistics Commands ──────────────────────────────────────────────────────

export async function getTodayStatistics(): Promise<TodayStatistics> {
  return invoke<TodayStatistics>('get_today_statistics')
}

export async function getRecentSessions(limit: number = 10): Promise<WorkSession[]> {
  return invoke<WorkSession[]>('get_recent_sessions', { limit })
}

export async function getStatisticsRange(startDate: string, endDate: string): Promise<TodayStatistics[]> {
  return invoke<TodayStatistics[]>('get_statistics_range', { startDate, endDate })
}

// ─── Event Listeners ─────────────────────────────────────────────────────────

export async function onActivityChanged(
  callback: (payload: ActivityChangedPayload) => void
): Promise<UnlistenFn> {
  return listen<ActivityChangedPayload>('activity://changed', (event) => {
    callback(event.payload)
  })
}

export async function onBreakDue(callback: () => void): Promise<UnlistenFn> {
  return listen('break://due', () => callback())
}

export async function onStatisticsUpdated(callback: () => void): Promise<UnlistenFn> {
  return listen('statistics://updated', () => callback())
}

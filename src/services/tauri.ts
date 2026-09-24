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

export interface AiChatMessage {
  role: 'user' | 'assistant'
  content: string
}

export interface AiSettingChange { key: string; oldValue: string; proposedValue: string }
export interface AiConfigurationProposal { changes: AiSettingChange[]; reason: string; impact: string; confidence: 'low' | 'medium' | 'high' | string }
export interface AiChatResponse { content: string; proposal: AiConfigurationProposal | null }
export interface AiConfigurationChange { id: number; reason: string; model: string; createdAtUtc: string; undoneAtUtc: string | null }

export interface ChatConversation {
  id: number
  title: string
  isArchived: boolean
  createdAtUtc: string
  updatedAtUtc: string
}

export interface StoredChatMessage extends AiChatMessage {
  id: number
  conversationId: number
  createdAtUtc: string
}

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
  if (settings.monitorPollIntervalSeconds !== undefined)
    await updateSetting('monitor.poll_interval_seconds', String(settings.monitorPollIntervalSeconds))
  if (settings.activityIdleCutoffSeconds !== undefined)
    await updateSetting('activity.idle_cutoff_seconds', String(settings.activityIdleCutoffSeconds))
  if (settings.breakEyeAfterMinutes !== undefined)
    await updateSetting('break.eye_after_minutes', String(settings.breakEyeAfterMinutes))
  if (settings.breakShortAfterMinutes !== undefined)
    await updateSetting('break.short_after_minutes', String(settings.breakShortAfterMinutes))
  if (settings.breakLongAfterMinutes !== undefined)
    await updateSetting('break.long_after_minutes', String(settings.breakLongAfterMinutes))
  if (settings.breakEyeDurationSeconds !== undefined)
    await updateSetting('break.eye_duration_seconds', String(settings.breakEyeDurationSeconds))
  if (settings.breakShortDurationSeconds !== undefined)
    await updateSetting('break.short_duration_seconds', String(settings.breakShortDurationSeconds))
  if (settings.breakLongDurationSeconds !== undefined)
    await updateSetting('break.long_duration_seconds', String(settings.breakLongDurationSeconds))
  if (settings.breakAutoCompleteIdleSeconds !== undefined)
    await updateSetting('break.auto_complete_idle_seconds', String(settings.breakAutoCompleteIdleSeconds))
  if (settings.notificationEnabled !== undefined)
    await updateSetting('notification.enabled', settings.notificationEnabled ? 'true' : 'false')
  if (settings.autostartEnabled !== undefined)
    await updateSetting('autostart.enabled', settings.autostartEnabled ? 'true' : 'false')
  if (settings.uiStartMinimized !== undefined)
    await updateSetting('ui.start_minimized', settings.uiStartMinimized ? 'true' : 'false')
  if (settings.privacyTrackForegroundApp !== undefined)
    await updateSetting('privacy.track_foreground_app', settings.privacyTrackForegroundApp ? 'true' : 'false')
  if (settings.aiEnabled !== undefined)
    await updateSetting('ai.enabled', settings.aiEnabled ? 'true' : 'false')
  if (settings.aiEndpoint !== undefined)
    await updateSetting('ai.endpoint', settings.aiEndpoint)
  if (settings.aiModel !== undefined)
    await updateSetting('ai.model', settings.aiModel)
}

export async function testAiConnection(endpoint: string, model: string): Promise<string> {
  return invoke<string>('test_ai_connection', { endpoint, model })
}

export async function chatWithAi(
  messages: AiChatMessage[],
  includeActivity: boolean,
  includeConfiguration: boolean,
  language: string,
): Promise<AiChatResponse> {
  return invoke<AiChatResponse>('chat_with_ai', { messages, includeActivity, includeConfiguration, language })
}

export async function applyAiConfigurationProposal(proposal: AiConfigurationProposal): Promise<number> {
  return invoke<number>('apply_ai_configuration_proposal', { input: { proposal } })
}
export async function getAiConfigurationChanges(): Promise<AiConfigurationChange[]> { return invoke('list_ai_configuration_changes') }
export async function undoAiConfigurationChange(changeId: number): Promise<void> { return invoke('undo_ai_configuration_change', { changeId }) }

export async function getChatConversations(archived = false): Promise<ChatConversation[]> {
  return invoke<ChatConversation[]>('list_chat_conversations', { archived })
}

export async function createChatConversation(title: string): Promise<ChatConversation> {
  return invoke<ChatConversation>('create_chat_conversation', { input: { title } })
}

export async function getChatMessages(conversationId: number): Promise<StoredChatMessage[]> {
  return invoke<StoredChatMessage[]>('get_chat_messages', { conversationId })
}

export async function saveChatMessage(conversationId: number, role: AiChatMessage['role'], content: string): Promise<StoredChatMessage> {
  return invoke<StoredChatMessage>('save_chat_message', { input: { conversationId, role, content } })
}

export async function archiveChatConversation(conversationId: number, archived: boolean): Promise<void> {
  return invoke('archive_chat_conversation', { input: { conversationId, archived } })
}

export async function deleteChatConversation(conversationId: number): Promise<void> {
  return invoke('delete_chat_conversation', { conversationId })
}

export async function deleteAllChatConversations(): Promise<void> {
  return invoke('delete_all_chat_conversations')
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

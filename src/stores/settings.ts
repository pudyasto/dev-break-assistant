// ─── Settings Store ───────────────────────────────────────────────────────────
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getSettings, updateSettings } from '@/services/tauri'
import type { AppSettings } from '@/types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings | null>(null)
  const loading = ref(false)
  const saving = ref(false)
  const error = ref<string | null>(null)

  const defaults: AppSettings = {
    monitorPollIntervalSeconds: 5,
    activityIdleCutoffSeconds: 60,
    breakEyeAfterMinutes: 20,
    breakShortAfterMinutes: 45,
    breakLongAfterMinutes: 90,
    breakEyeDurationSeconds: 20,
    breakShortDurationSeconds: 180,
    breakLongDurationSeconds: 300,
    breakAutoCompleteIdleSeconds: 180,
    notificationEnabled: true,
    autostartEnabled: false,
    uiStartMinimized: false,
    privacyTrackForegroundApp: false,
    aiEnabled: false,
    aiEndpoint: 'http://localhost:11434/api/generate',
    aiModel: 'llama3',
  }

  async function fetchSettings() {
    loading.value = true
    error.value = null
    try {
      settings.value = await getSettings()
    } catch (e) {
      error.value = String(e)
      settings.value = { ...defaults }
    } finally {
      loading.value = false
    }
  }

  async function saveSettings(partial: Partial<AppSettings>) {
    saving.value = true
    error.value = null
    try {
      await updateSettings(partial)
      if (settings.value) {
        Object.assign(settings.value, partial)
      }
    } catch (e) {
      error.value = String(e)
      throw e
    } finally {
      saving.value = false
    }
  }

  return { settings, loading, saving, error, defaults, fetchSettings, saveSettings }
})

// ─── Activity Store ───────────────────────────────────────────────────────────
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  getCurrentActivity,
  getPlatformCapabilities,
  onActivityChanged,
  pauseMonitoring as apiPauseMonitoring,
  resumeMonitoring as apiResumeMonitoring,
  startBreak as apiStartBreak,
  completeBreak as apiCompleteBreak,
  snoozeReminder as apiSnoozeReminder,
  dismissReminder as apiDismissReminder,
} from '@/services/tauri'
import type { CurrentActivity, PlatformCapabilities, ActivityState } from '@/types'
import { stateLabel, stateColor } from '@/types'
import i18n from '@/i18n'

export const useActivityStore = defineStore('activity', () => {
  // ─── State
  const activity = ref<CurrentActivity | null>(null)
  const capabilities = ref<PlatformCapabilities | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const isPaused = ref(false)

  // Local tick counter for smooth UI counter (1s interval)
  const localIdleSeconds = ref(0)
  const localActiveSeconds = ref(0)
  let tickInterval: ReturnType<typeof setInterval> | null = null
  let unlistenActivity: (() => void) | null = null

  // ─── Computed
  const state = computed((): ActivityState => {
    if (isPaused.value) return 'Paused'
    return activity.value?.state ?? 'Initializing'
  })
  const idleSeconds = computed(() => localIdleSeconds.value)
  const activeSeconds = computed(() => localActiveSeconds.value)
  const stateText = computed(() => {
    i18n.global.locale.value
    return i18n.global.t(`status.${state.value.charAt(0).toLowerCase()}${state.value.slice(1)}`, stateLabel(state.value))
  })
  const stateClass = computed(() => stateColor(state.value))

  // ─── Actions
  async function fetchActivity() {
    try {
      activity.value = await getCurrentActivity()
      localIdleSeconds.value = activity.value.idleSeconds
      localActiveSeconds.value = activity.value.activeSessionSeconds
      if (activity.value.state === 'Paused') {
        isPaused.value = true
      }
    } catch (e) {
      error.value = String(e)
    }
  }

  async function fetchCapabilities() {
    try {
      capabilities.value = await getPlatformCapabilities()
    } catch (e) {
      // Capabilities may not be available in early phases — silent fail
      console.warn('Could not fetch platform capabilities:', e)
    }
  }

  async function pause() {
    try {
      await apiPauseMonitoring()
      isPaused.value = true
      if (activity.value) activity.value.state = 'Paused'
    } catch (e) {
      error.value = String(e)
    }
  }

  async function resume() {
    try {
      await apiResumeMonitoring()
      isPaused.value = false
      await fetchActivity()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function togglePause() {
    if (isPaused.value || state.value === 'Paused') {
      await resume()
    } else {
      await pause()
    }
  }

  async function triggerBreak(breakType: string) {
    try {
      await apiStartBreak(breakType)
      if (activity.value) activity.value.state = 'Breaking'
      await fetchActivity()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function finishBreak(breakType: string, durationSeconds: number) {
    try {
      await apiCompleteBreak(breakType, durationSeconds)
      if (activity.value) {
        activity.value.state = 'Active'
        activity.value.activeSessionSeconds = 0
      }
      localActiveSeconds.value = 0
      await fetchActivity()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function snooze(breakType: string) {
    try {
      await apiSnoozeReminder(breakType)
      await fetchActivity()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function dismiss(breakType: string) {
    try {
      await apiDismissReminder(breakType)
      await fetchActivity()
    } catch (e) {
      error.value = String(e)
    }
  }

  async function startListening() {
    loading.value = true
    await fetchCapabilities()
    await fetchActivity()
    loading.value = false

    // Listen for backend events
    unlistenActivity = await onActivityChanged((payload) => {
      if (activity.value) {
        activity.value.state = payload.state
        activity.value.idleSeconds = payload.idleSeconds
        activity.value.activeSessionSeconds = payload.activeSessionSeconds
      }
      localIdleSeconds.value = payload.idleSeconds
      localActiveSeconds.value = payload.activeSessionSeconds
      if (payload.state === 'Paused') isPaused.value = true
      else if (isPaused.value && payload.state === 'Active') isPaused.value = false
    })

    // Smooth UI ticker (1s) — does NOT query the database
    tickInterval = setInterval(() => {
      if (state.value === 'Active') {
        localActiveSeconds.value += 1
        localIdleSeconds.value = 0
      } else if (state.value === 'Idle') {
        localIdleSeconds.value += 1
      }
    }, 1000)
  }

  function stopListening() {
    if (tickInterval) { clearInterval(tickInterval); tickInterval = null }
    if (unlistenActivity) { unlistenActivity(); unlistenActivity = null }
  }

  return {
    activity,
    capabilities,
    loading,
    error,
    isPaused,
    state,
    idleSeconds,
    activeSeconds,
    stateText,
    stateClass,
    fetchActivity,
    fetchCapabilities,
    pause,
    resume,
    togglePause,
    triggerBreak,
    finishBreak,
    snooze,
    dismiss,
    startListening,
    stopListening,
  }
})

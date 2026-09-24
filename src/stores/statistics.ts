// ─── Statistics Store ─────────────────────────────────────────────────────────
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getTodayStatistics, getRecentSessions, getStatisticsRange, onStatisticsUpdated } from '@/services/tauri'
import type { TodayStatistics, WorkSession } from '@/types'

export const useStatisticsStore = defineStore('statistics', () => {
  const todayStats = ref<TodayStatistics | null>(null)
  const rangeStats = ref<TodayStatistics[]>([])
  const recentSessions = ref<WorkSession[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  let unlistenStats: (() => void) | null = null

  async function fetchRange(startDate: string, endDate: string) {
    try {
      rangeStats.value = await getStatisticsRange(startDate, endDate)
    } catch (e) {
      console.warn('Could not fetch range stats:', e)
    }
  }

  async function fetchToday() {
    loading.value = true
    error.value = null
    try {
      todayStats.value = await getTodayStatistics()
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function fetchRecentSessions() {
    try {
      recentSessions.value = await getRecentSessions(10)
    } catch (e) {
      console.warn('Could not fetch recent sessions:', e)
    }
  }

  async function startListening() {
    await fetchToday()
    await fetchRecentSessions()

    unlistenStats = await onStatisticsUpdated(async () => {
      await fetchToday()
    })
  }

  function stopListening() {
    if (unlistenStats) { unlistenStats(); unlistenStats = null }
  }

  return {
    todayStats,
    rangeStats,
    recentSessions,
    loading,
    error,
    fetchToday,
    fetchRange,
    fetchRecentSessions,
    startListening,
    stopListening,
  }
})

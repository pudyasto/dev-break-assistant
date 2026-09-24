<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">

    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('statistics.title') }}</h1>
        <p class="text-[14px] text-muted mt-1">{{ t('statistics.subtitle') }}</p>
      </div>

      <div class="flex bg-gray-100 dark:bg-gray-800 p-1 rounded-lg gap-1">
        <button
          v-for="opt in ['today', '7_days', '30_days']"
          :key="opt"
          @click="selectedView = opt"
          class="px-4 py-1.5 text-[13px] font-medium rounded-md transition-colors"
          :class="selectedView === opt ? 'bg-white dark:bg-gray-700 text-[var(--color-text)] shadow-sm' : 'text-muted hover:text-[var(--color-text)]'"
        >
          {{ opt === 'today' ? t('statistics.today') : opt === '7_days' ? t('statistics.sevenDays') : t('statistics.thirtyDays') }}
        </button>
      </div>
    </div>

    <!-- Data Overview -->
    <div v-if="aggregatedStats">
      <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">
        {{ viewLabel }}
      </h2>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <StatCard :label="t('dashboard.activeTime')" :value="fmt(aggregatedStats.activeSeconds)" :delay="0" />
        <StatCard :label="t('statistics.idleTime')" :value="fmt(aggregatedStats.idleSeconds)" :delay="1" />
        <StatCard :label="t('dashboard.breakTime')" :value="fmt(aggregatedStats.breakSeconds)" :delay="2" />
        <StatCard :label="t('statistics.totalBreaks')" :value="String(aggregatedStats.breakCount)" :delay="3" />
        <StatCard :label="t('statistics.eyeBreaks')" :value="String(aggregatedStats.eyeBreakCount)" :delay="4" />
        <StatCard :label="t('statistics.movementBreaks')" :value="String(aggregatedStats.shortBreakCount)" :delay="5" />
        <StatCard :label="t('statistics.longBreaks')" :value="String(aggregatedStats.longBreakCount)" :delay="6" />
        <StatCard :label="t('dashboard.longestStreak')" :value="fmt(aggregatedStats.longestActiveStreakSeconds)" :delay="7" />
        <StatCard :label="t('statistics.skipped')" :value="String(aggregatedStats.skippedBreakCount)" :delay="8" />
        <StatCard :label="t('statistics.snoozed')" :value="String(aggregatedStats.snoozedReminderCount)" :delay="9" />
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="card p-12 flex flex-col items-center gap-3 text-center">
      <p class="font-semibold text-[var(--color-text)]">{{ t('statistics.emptyTitle') }}</p>
      <p class="text-[13px] text-muted">{{ t('statistics.emptyDescription') }}</p>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useStatisticsStore } from '@/stores/statistics'
import { formatDuration } from '@/types'
import type { TodayStatistics } from '@/types'
import StatCard from '@/components/StatCard.vue'
import { useI18n } from 'vue-i18n'

const statsStore = useStatisticsStore()
const { t } = useI18n()
const fmt = (s: number) => formatDuration(s)

const selectedView = ref('today')

onMounted(async () => {
  await statsStore.fetchToday()
  
  const today = new Date()
  const past30 = new Date()
  past30.setDate(today.getDate() - 30)
  
  const fmtDate = (d: Date) => d.toISOString().split('T')[0]
  await statsStore.fetchRange(fmtDate(past30), fmtDate(today))
})

const viewLabel = computed(() => {
  if (selectedView.value === 'today') return t('statistics.todayLabel', { date: statsStore.todayStats?.localDate || '...' })
  if (selectedView.value === '7_days') return t('statistics.lastSeven')
  return t('statistics.lastThirty')
})

const aggregatedStats = computed<TodayStatistics | null>(() => {
  if (selectedView.value === 'today') {
    return statsStore.todayStats
  }
  
  if (statsStore.rangeStats.length === 0) return null
  
  const days = selectedView.value === '7_days' ? 7 : 30
  
  const today = new Date()
  const limitDate = new Date()
  limitDate.setDate(today.getDate() - days)
  const limitStr = limitDate.toISOString().split('T')[0]
  
  const relevantStats = statsStore.rangeStats.filter(s => s.localDate >= limitStr)
  
  if (relevantStats.length === 0) return null
  
  return relevantStats.reduce((acc, curr) => ({
    localDate: 'Aggregate',
    activeSeconds: acc.activeSeconds + curr.activeSeconds,
    idleSeconds: acc.idleSeconds + curr.idleSeconds,
    breakSeconds: acc.breakSeconds + curr.breakSeconds,
    breakCount: acc.breakCount + curr.breakCount,
    eyeBreakCount: acc.eyeBreakCount + curr.eyeBreakCount,
    shortBreakCount: acc.shortBreakCount + curr.shortBreakCount,
    longBreakCount: acc.longBreakCount + curr.longBreakCount,
    skippedBreakCount: acc.skippedBreakCount + curr.skippedBreakCount,
    snoozedReminderCount: acc.snoozedReminderCount + curr.snoozedReminderCount,
    longestActiveStreakSeconds: Math.max(acc.longestActiveStreakSeconds, curr.longestActiveStreakSeconds),
    deskHabitScore: null
  }), {
    localDate: 'Aggregate',
    activeSeconds: 0,
    idleSeconds: 0,
    breakSeconds: 0,
    breakCount: 0,
    eyeBreakCount: 0,
    shortBreakCount: 0,
    longBreakCount: 0,
    skippedBreakCount: 0,
    snoozedReminderCount: 0,
    longestActiveStreakSeconds: 0,
    deskHabitScore: null
  })
})
</script>

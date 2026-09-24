<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">
    <div>
      <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('history.title') }}</h1>
      <p class="text-[14px] text-muted mt-1">{{ t('history.subtitle') }}</p>
    </div>

    <div v-if="statsStore.recentSessions.length === 0" class="card p-12 flex flex-col items-center gap-3 text-center">
      <p class="font-semibold text-[var(--color-text)]">{{ t('history.emptyTitle') }}</p>
      <p class="text-[13px] text-muted">{{ t('history.emptyDescription') }}</p>
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="session in statsStore.recentSessions"
        :key="session.id"
        class="card-hover p-5 flex items-center justify-between gap-4"
      >
        <div class="flex items-center gap-4">
          <div class="w-2 h-2 rounded-full" :class="statusDot(session.status)"></div>
          <div>
            <p class="text-[14px] font-medium text-[var(--color-text)]">
              {{ formatSessionTime(session.startedAtUtc) }}
            </p>
            <p class="text-[12px] text-muted">{{ t('history.activeDuration', { duration: fmt(session.activeSeconds) }) }}</p>
          </div>
        </div>
        <span class="badge" :class="sessionBadge(session.status)">{{ session.status }}</span>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStatisticsStore } from '@/stores/statistics'
import { formatDuration } from '@/types'
import { useI18n } from 'vue-i18n'


const statsStore = useStatisticsStore()
const { t, locale } = useI18n()
const fmt = (s: number) => formatDuration(s)

onMounted(() => statsStore.fetchRecentSessions())

function statusDot(status: string) {
  if (status === 'running') return 'bg-green-500'
  if (status === 'completed') return 'bg-gray-500'
  return 'bg-amber-500'
}

function sessionBadge(status: string) {
  if (status === 'running') return 'badge-success'
  if (status === 'completed') return 'badge-muted'
  return 'badge-warning'
}

function formatSessionTime(utc: string) {
  return new Intl.DateTimeFormat(locale.value === 'id' ? 'id-ID' : 'en-US', {
    weekday: 'short', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit',
  }).format(new Date(utc))
}
</script>

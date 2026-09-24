<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">

    <!-- ── Header ─────────────────────────────────────────────────── -->
    <div class="flex items-start justify-between">
      <div>
        <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('dashboard.greeting') }}</h1>
        <p class="text-[14px] text-muted mt-1">{{ t('dashboard.subtitle') }}</p>
        <p class="text-[14px] text-muted mt-2">{{ currentDate }}</p>
      </div>
      <div class="flex flex-col items-end gap-4">
        <button class="btn-primary" @click="router.push('/activity')">{{ t('dashboard.takeBreak') }}</button>
        <div class="flex items-center gap-2">
          <span class="glow-dot glow-dot-green"></span>
          <span class="text-[13px] font-medium text-[var(--color-text)]">{{ t('status.monitoring') }}</span>
        </div>
      </div>
    </div>

    <!-- ── Loading state ──────────────────────────────────────────── -->
    <div v-if="activityStore.loading" class="flex items-center justify-center py-20">
      <div class="flex flex-col items-center gap-4">
        <div class="w-8 h-8 rounded-full border-2 border-[var(--color-accent)] border-t-transparent animate-spin"></div>
        <span class="text-sm text-muted">{{ t('dashboard.connecting') }}</span>
      </div>
    </div>

    <template v-else>
      <!-- ── AI Tip ─────────────────────────────────────────────────── -->
      <div v-if="settings.aiEnabled" class="card bg-[var(--color-surface)] border border-[var(--color-accent)]/30 p-4 flex items-start justify-between gap-4 animate-fade-in shadow-sm shadow-[var(--color-accent)]/5">
        <div class="flex items-start gap-3 min-w-0">
          <div class="text-[20px] select-none">✨</div>
          <div class="space-y-1 min-w-0">
            <div class="flex items-center gap-2">
              <h3 class="text-xs font-semibold uppercase tracking-widest text-[var(--color-accent)]">{{ t('dashboard.aiCoach') }}</h3>
              <span v-if="loadingTip" class="text-[11px] text-muted animate-pulse">{{ t('dashboard.aiLoading') }}</span>
            </div>
            <MarkdownContent v-if="aiTip" :content="aiTip" class="text-sm text-[var(--color-text)] leading-relaxed" />
            <p v-else-if="aiTipError" class="text-xs text-red-400 leading-relaxed">{{ aiTipError }}</p>
            <p v-else-if="!loadingTip" class="text-xs text-muted">{{ t('dashboard.aiEmpty') }}</p>
          </div>
        </div>

        <button
          type="button"
          class="btn-ghost text-xs p-2 shrink-0 text-muted hover:text-[var(--color-text)] rounded-lg"
          :title="loadingTip ? t('dashboard.aiRefreshing') : t('dashboard.aiRefresh')"
          :disabled="loadingTip"
          @click="fetchAiTip"
        >
          <svg class="w-4 h-4" :class="{ 'animate-spin': loadingTip }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>
      </div>

      <!-- ── Current Session ───────────────────────────────────────── -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

        <!-- Activity ring timer -->
        <ActivityTimer
          :state="activityStore.state"
          :seconds="activityStore.activeSeconds"
          :max-seconds="shortBreakThresholdSec"
          :subtitle="t('dashboard.activeSession')"
        />

        <!-- Idle + state info -->
        <div class="card p-6 flex flex-col justify-between gap-4">
          <!-- State badge -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold uppercase tracking-widest text-muted">{{ t('dashboard.currentState') }}</span>
            <span class="badge" :class="stateBadgeClass">
              <span class="glow-dot text-[10px]" :class="dotClass"></span>
              {{ activityStore.stateText }}
            </span>
          </div>

          <!-- Idle counter -->
          <div class="flex flex-col gap-1">
            <span class="text-xs font-semibold uppercase tracking-widest text-muted">{{ t('dashboard.idleTime') }}</span>
            <div class="flex items-end gap-2">
              <span class="text-4xl font-bold font-mono text-[var(--color-text)] animate-counter tracking-tight">
                {{ formatDuration(activityStore.idleSeconds) }}
              </span>
            </div>
            <div class="progress-bar mt-3">
              <div
                class="progress-fill"
                :style="{
                  width: `${idleProgress}%`,
                  background: idleProgress > 80 ? 'var(--color-green)' : 'var(--color-amber)'
                }"
              ></div>
            </div>
            <span class="text-[13px] text-muted mt-2">
              {{ idleProgress >= 100 ? t('dashboard.autoBreak') : t('dashboard.untilAutoBreak', { seconds: idleCutoffSec }) }}
            </span>
          </div>

          <!-- Platform capability detail -->
          <div v-if="capabilities" class="border-t border-app pt-4">
            <div class="grid grid-cols-2 gap-y-2 gap-x-4 mt-2">
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities.idleDetection ? 'glow-dot-green' : 'glow-dot-amber'"></span>
                <span class="text-[12px] text-muted">{{ t('dashboard.idleDetection') }}</span>
              </div>
              <div class="text-[12px] font-medium" :class="capabilities.idleDetection ? 'text-green-500' : 'text-amber-500'">
                {{ capabilities.idleDetection ? t('common.available') : t('common.fallback') }}
              </div>
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities.sessionLockDetection ? 'glow-dot-green' : 'glow-dot-muted'"></span>
                <span class="text-[12px] text-muted">{{ t('dashboard.screenLock') }}</span>
              </div>
              <div class="text-[12px] font-medium" :class="capabilities.sessionLockDetection ? 'text-green-500' : 'text-muted'">
                {{ capabilities.sessionLockDetection ? t('common.available') : t('common.unavailable') }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- ── Today Stats ────────────────────────────────────────────── -->
      <div>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('dashboard.overview') }}</h2>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <StatCard
            :label="t('dashboard.activeTime')"
            :value="formatDuration(todayStats?.activeSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="0"
          />
          <StatCard
            :label="t('dashboard.breakTime')"
            :value="formatDuration(todayStats?.breakSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="1"
          />
          <StatCard
            :label="t('dashboard.breaksTaken')"
            :value="String(todayStats?.breakCount ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="2"
          />
          <StatCard
            :label="t('dashboard.longestStreak')"
            :value="formatDuration(todayStats?.longestActiveStreakSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="3"
          />
        </div>
      </div>

      <!-- ── Break Schedule ────────────────────────────────────────── -->
      <div>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('dashboard.nextBreak') }}</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <BreakCard
            :title="t('dashboard.eyeRest')"
            :description="t('dashboard.inSeconds', { seconds: activityStore.activity?.nextEyeBreakSeconds ?? 0 })"
            icon=""
            icon-bg=""
            :countdown-seconds="activityStore.activity?.nextEyeBreakSeconds ?? null"
            :progress="eyeProgress"
            progress-color="var(--color-blue)"
          />
          <BreakCard
            :title="t('dashboard.movementBreak')"
            :description="t('dashboard.inSeconds', { seconds: activityStore.activity?.nextShortBreakSeconds ?? 0 })"
            icon=""
            icon-bg=""
            :countdown-seconds="activityStore.activity?.nextShortBreakSeconds ?? null"
            :progress="shortProgress"
            progress-color="var(--color-green)"
          />
          <BreakCard
            :title="t('dashboard.longBreak')"
            :description="t('dashboard.inSeconds', { seconds: activityStore.activity?.nextLongBreakSeconds ?? 0 })"
            icon=""
            icon-bg=""
            :countdown-seconds="activityStore.activity?.nextLongBreakSeconds ?? null"
            :progress="longProgress"
            progress-color="var(--color-amber)"
          />
        </div>
      </div>
    </template>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useActivityStore } from '@/stores/activity'
import { useStatisticsStore } from '@/stores/statistics'
import { useSettingsStore } from '@/stores/settings'
import { formatDuration } from '@/types'
import ActivityTimer from '@/components/ActivityTimer.vue'
import BreakCard from '@/components/BreakCard.vue'
import StatCard from '@/components/StatCard.vue'
import MarkdownContent from '@/components/MarkdownContent.vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const activityStore = useActivityStore()
const statsStore = useStatisticsStore()
const settingsStore = useSettingsStore()
const { t, locale } = useI18n()

const aiTip = ref<string | null>(null)
const loadingTip = ref(false)
const aiTipError = ref<string | null>(null)

async function fetchAiTip() {
  if (!settings.value.aiEnabled) return
  loadingTip.value = true
  aiTipError.value = null
  try {
    const tip = await invoke<string | null>('generate_daily_tip', { language: locale.value })
    if (tip) {
      aiTip.value = tip
    } else {
      aiTip.value = t('dashboard.aiFallback')
    }
  } catch (e: any) {
    aiTipError.value = t('dashboard.aiError', { error: e?.message || e })
  } finally {
    loadingTip.value = false
  }
}

// ─── Lifecycle
onMounted(async () => {
  await activityStore.startListening()
  await statsStore.startListening()
  if (!settingsStore.settings) await settingsStore.fetchSettings()
  
  if (settingsStore.settings?.aiEnabled) {
    fetchAiTip()
  }
})
onUnmounted(() => {
  activityStore.stopListening()
  statsStore.stopListening()
})

// ─── Computed
const capabilities = computed(() => activityStore.capabilities)
const todayStats = computed(() => statsStore.todayStats)
const settings = computed(() => settingsStore.settings ?? settingsStore.defaults)

const eyeAfterMin = computed(() => settings.value.breakEyeAfterMinutes)
const shortAfterMin = computed(() => settings.value.breakShortAfterMinutes)
const longAfterMin = computed(() => settings.value.breakLongAfterMinutes)
const shortBreakThresholdSec = computed(() => shortAfterMin.value * 60)
const idleCutoffSec = computed(() => settings.value.activityIdleCutoffSeconds)

const idleProgress = computed(() =>
  Math.min(100, (activityStore.idleSeconds / idleCutoffSec.value) * 100)
)

const eyeProgress = computed(() => {
  const next = activityStore.activity?.nextEyeBreakSeconds
  if (next == null) return 0
  const total = eyeAfterMin.value * 60
  return Math.min(100, ((total - next) / total) * 100)
})

const shortProgress = computed(() => {
  const next = activityStore.activity?.nextShortBreakSeconds
  if (next == null) return 0
  const total = shortAfterMin.value * 60
  return Math.min(100, ((total - next) / total) * 100)
})

const longProgress = computed(() => {
  const next = activityStore.activity?.nextLongBreakSeconds
  if (next == null) return 0
  const total = longAfterMin.value * 60
  return Math.min(100, ((total - next) / total) * 100)
})

const stateBadgeClass = computed(() => {
  const s = activityStore.state
  if (s === 'Active')   return 'badge-success'
  if (s === 'Idle')     return 'badge-warning'
  if (s === 'BreakDue') return 'badge-danger'
  return 'badge-muted'
})

const dotClass = computed(() => {
  const s = activityStore.state
  if (s === 'Active')   return 'glow-dot-green'
  if (s === 'Idle')     return 'glow-dot-amber'
  if (s === 'BreakDue') return 'glow-dot-red'
  return 'glow-dot-muted'
})

const currentDate = computed(() =>
  new Intl.DateTimeFormat(locale.value === 'id' ? 'id-ID' : 'en-US', { weekday: 'long', month: 'long', day: 'numeric' }).format(new Date())
)
</script>

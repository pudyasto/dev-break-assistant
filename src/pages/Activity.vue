<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">

    <!-- ── Header ─────────────────────────────────────────────────────────── -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-3">
          <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('nav.activity') }}</h1>
          <span class="badge" :class="stateBadgeClass">
            <span class="glow-dot text-[10px]" :class="dotClass"></span>
            {{ activityStore.stateText }}
          </span>
        </div>
        <p class="text-[14px] text-muted mt-1">
          {{ t('activity.subtitle') }}
        </p>
      </div>

      <div class="flex items-center gap-3">
        <button
          @click="toggleMonitoring"
          class="btn text-sm font-medium transition-all"
          :class="isPaused ? 'btn-primary' : 'bg-[var(--color-surface-elev)] text-[var(--color-text)] border border-[var(--color-border)] hover:bg-[var(--color-hover)]'"
        >
          <svg v-if="isPaused" xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="5 3 19 12 5 21 5 3"></polygon>
          </svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="6" y="4" width="4" height="16"></rect>
            <rect x="14" y="4" width="4" height="16"></rect>
          </svg>
          <span>{{ isPaused ? t('activity.resume') : t('activity.pause') }}</span>
        </button>

        <button
          @click="refreshData"
          class="btn bg-[var(--color-surface)] border border-[var(--color-border)] text-muted hover:text-[var(--color-text)] hover:bg-[var(--color-hover)]"
          :title="t('activity.refreshTitle')"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :class="{ 'animate-spin': isRefreshing }">
            <polyline points="23 4 23 10 17 10"></polyline>
            <polyline points="1 20 1 14 7 14"></polyline>
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
          </svg>
          <span>{{ t('common.refresh') }}</span>
        </button>
      </div>
    </div>

    <!-- ── Active Break Banner (if currently taking a break) ─────────────── -->
    <div
      v-if="isBreaking || activeManualBreak"
      class="card p-6 border-[var(--color-accent)] bg-[var(--color-surface-elev)] shadow-lg shadow-[var(--color-accent)]/10 animate-slide-up"
    >
      <div class="flex flex-col md:flex-row items-center justify-between gap-6">
        <div class="flex items-center gap-5">
          <div class="w-14 h-14 rounded-2xl bg-[var(--color-accent)]/20 border border-[var(--color-accent)]/40 flex items-center justify-center text-3xl">
            {{ activeManualBreak?.icon || '🧘' }}
          </div>
          <div>
            <div class="flex items-center gap-2">
              <span class="badge badge-success">{{ t('activity.breakProgress') }}</span>
              <span class="text-xs text-muted">{{ activeManualBreak?.title || t('activity.restSession') }}</span>
            </div>
            <h2 class="text-xl font-bold text-[var(--color-text)] mt-1">
              {{ t('activity.relaxTitle') }}
            </h2>
            <p class="text-sm text-muted mt-0.5">
              {{ activeManualBreak?.hint || t('activity.relaxHint') }}
            </p>
          </div>
        </div>

        <div class="flex flex-col items-center sm:items-end gap-3 w-full sm:w-auto">
          <div class="text-3xl font-mono font-bold text-[var(--color-text)] animate-counter">
            {{ formatSeconds(breakCountdown) }}
          </div>
          <div class="flex items-center gap-2">
            <button @click="finishActiveBreak" class="btn-primary text-xs py-1.5 px-3">
              {{ t('activity.complete') }}
            </button>
            <button @click="snoozeActiveBreak" class="btn bg-[var(--color-surface)] border border-[var(--color-border)] text-muted hover:text-[var(--color-text)] text-xs py-1.5 px-3">
              {{ t('activity.snooze') }}
            </button>
            <button @click="cancelActiveBreak" class="btn text-muted hover:text-[var(--color-red)] text-xs py-1.5 px-2">
              {{ t('activity.dismiss') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- ── Live Metrics Section ─────────────────────────────────────────── -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

      <!-- Active Timer Ring Card -->
      <ActivityTimer
        :state="activityStore.state"
        :seconds="activityStore.activeSeconds"
        :max-seconds="shortBreakThresholdSec"
        :subtitle="t('activity.currentSession')"
      />

      <!-- Idle Counter & Status Card -->
      <div class="card p-6 flex flex-col justify-between gap-5">
        <div>
          <div class="flex items-center justify-between mb-3">
            <span class="text-xs font-semibold uppercase tracking-widest text-muted">{{ t('activity.detector') }}</span>
            <span class="text-xs font-mono text-muted">
              {{ t('activity.threshold', { seconds: idleCutoffSec }) }}
            </span>
          </div>

          <div class="flex items-baseline gap-3">
            <span class="text-4xl font-bold font-mono text-[var(--color-text)] animate-counter tracking-tight">
              {{ formatDuration(activityStore.idleSeconds) }}
            </span>
            <span class="text-xs text-muted font-medium">{{ t('activity.away') }}</span>
          </div>

          <!-- Progress bar towards auto-break cutoff -->
          <div class="progress-bar mt-4">
            <div
              class="progress-fill"
              :style="{
                width: `${idleProgress}%`,
                background: idleProgress >= 100 ? 'var(--color-green)' : idleProgress > 50 ? 'var(--color-amber)' : 'var(--color-accent)'
              }"
            ></div>
          </div>

          <div class="flex justify-between items-center text-[12px] text-muted mt-2">
            <span>{{ idleProgress >= 100 ? t('activity.autoTriggered') : t('activity.remaining', { seconds: Math.max(0, idleCutoffSec - activityStore.idleSeconds) }) }}</span>
            <span>{{ Math.round(idleProgress) }}%</span>
          </div>
        </div>

        <!-- Session Status Info -->
        <div class="border-t border-app pt-4 space-y-2.5">
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted">{{ t('activity.totalActive') }}</span>
            <span class="font-medium text-[var(--color-text)] font-mono">{{ formatDuration(statsStore.todayStats?.activeSeconds ?? 0) }}</span>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted">{{ t('activity.completedToday') }}</span>
            <span class="font-medium text-[var(--color-green)] font-mono">{{ t('activity.breakCount', { count: statsStore.todayStats?.breakCount ?? 0 }) }}</span>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted">{{ t('activity.focusStreak') }}</span>
            <span class="font-medium text-[var(--color-text)] font-mono">{{ formatDuration(statsStore.todayStats?.longestActiveStreakSeconds ?? 0) }}</span>
          </div>
        </div>
      </div>

    </div>

    <!-- ── Manual Break Launcher ────────────────────────────────────────── -->
    <div>
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="text-xs font-semibold uppercase tracking-widest text-muted">{{ t('activity.quickBreaks') }}</h2>
          <p class="text-xs text-muted mt-0.5">{{ t('activity.quickBreaksHint') }}</p>
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <!-- 20-20-20 Eye Break -->
        <div class="card p-5 flex flex-col justify-between gap-4 hover:border-[var(--color-accent)]/50 transition-all group">
          <div class="flex items-start justify-between">
            <div class="w-10 h-10 rounded-xl bg-blue-500/10 text-blue-400 flex items-center justify-center text-xl">
              👁️
            </div>
            <span class="text-xs font-mono text-muted bg-[var(--color-surface-elev)] px-2 py-0.5 rounded">20 sec</span>
          </div>
          <div>
            <h3 class="font-semibold text-[15px] text-[var(--color-text)] group-hover:text-[var(--color-accent)] transition-colors">
              {{ t('activity.eyeTitle') }}
            </h3>
            <p class="text-[13px] text-muted mt-1 leading-relaxed">
              {{ t('activity.eyeDesc') }}
            </p>
          </div>
          <button
            @click="startManualBreak('eye', 20, t('activity.eyeTitle'), '👁️', t('activity.eyeDesc'))"
            class="btn bg-[var(--color-surface-elev)] hover:bg-[var(--color-accent)] hover:text-white text-xs py-2 w-full justify-center transition-all"
          >
            {{ t('activity.startEye') }}
          </button>
        </div>

        <!-- Movement Break -->
        <div class="card p-5 flex flex-col justify-between gap-4 hover:border-[var(--color-green)]/50 transition-all group">
          <div class="flex items-start justify-between">
            <div class="w-10 h-10 rounded-xl bg-green-500/10 text-green-400 flex items-center justify-center text-xl">
              🧘
            </div>
            <span class="text-xs font-mono text-muted bg-[var(--color-surface-elev)] px-2 py-0.5 rounded">3 min</span>
          </div>
          <div>
            <h3 class="font-semibold text-[15px] text-[var(--color-text)] group-hover:text-[var(--color-green)] transition-colors">
              {{ t('dashboard.movementBreak') }}
            </h3>
            <p class="text-[13px] text-muted mt-1 leading-relaxed">
              {{ t('activity.movementDesc') }}
            </p>
          </div>
          <button
            @click="startManualBreak('short', 180, t('dashboard.movementBreak'), '🧘', t('activity.movementDesc'))"
            class="btn bg-[var(--color-surface-elev)] hover:bg-[var(--color-green)] hover:text-white text-xs py-2 w-full justify-center transition-all"
          >
            {{ t('activity.startMovement') }}
          </button>
        </div>

        <!-- Long Rest Break -->
        <div class="card p-5 flex flex-col justify-between gap-4 hover:border-[var(--color-amber)]/50 transition-all group">
          <div class="flex items-start justify-between">
            <div class="w-10 h-10 rounded-xl bg-amber-500/10 text-amber-400 flex items-center justify-center text-xl">
              ☕
            </div>
            <span class="text-xs font-mono text-muted bg-[var(--color-surface-elev)] px-2 py-0.5 rounded">10 min</span>
          </div>
          <div>
            <h3 class="font-semibold text-[15px] text-[var(--color-text)] group-hover:text-[var(--color-amber)] transition-colors">
              {{ t('activity.deepRest') }}
            </h3>
            <p class="text-[13px] text-muted mt-1 leading-relaxed">
              {{ t('activity.deepRestDesc') }}
            </p>
          </div>
          <button
            @click="startManualBreak('long', 600, t('activity.deepRest'), '☕', t('activity.deepRestDesc'))"
            class="btn bg-[var(--color-surface-elev)] hover:bg-[var(--color-amber)] hover:text-white text-xs py-2 w-full justify-center transition-all"
          >
            {{ t('activity.startLong') }}
          </button>
        </div>
      </div>
    </div>

    <!-- ── Platform Telemetry & Privacy Sensor Status ───────────────────── -->
    <div>
      <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('activity.telemetry') }}</h2>
      <div class="card p-6">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">

          <!-- Idle Detection Sensor -->
          <div class="flex flex-col justify-between space-y-2">
            <div>
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities?.idleDetection ? 'glow-dot-green' : 'glow-dot-amber'"></span>
                <span class="text-sm font-semibold text-[var(--color-text)]">Idle Detection Engine</span>
              </div>
              <p class="text-xs text-muted mt-1">
                {{ capabilities?.idleDetection ? 'Native Mutter D-Bus idle monitor active' : 'Fallback idle detection active' }}
              </p>
            </div>
            <div class="text-xs font-medium" :class="capabilities?.idleDetection ? 'text-green-400' : 'text-amber-400'">
              {{ capabilities?.idleDetection ? 'Operating Normally' : 'Fallback Mode' }}
            </div>
          </div>

          <!-- Screen Lock Sensor -->
          <div class="flex flex-col justify-between space-y-2 border-t md:border-t-0 md:border-l border-app pt-4 md:pt-0 md:pl-6">
            <div>
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities?.sessionLockDetection ? 'glow-dot-green' : 'glow-dot-muted'"></span>
                <span class="text-sm font-semibold text-[var(--color-text)]">Screen Lock Monitor</span>
              </div>
              <p class="text-xs text-muted mt-1">
                {{ capabilities?.sessionLockDetection ? 'D-Bus ScreenSaver / login1 auto-pause' : 'Not supported on current display manager' }}
              </p>
            </div>
            <div class="text-xs font-medium" :class="capabilities?.sessionLockDetection ? 'text-green-400' : 'text-muted'">
              {{ capabilities?.sessionLockDetection ? 'Active & Watching' : 'Unavailable' }}
            </div>
          </div>

          <!-- Foreground App Tracking -->
          <div class="flex flex-col justify-between space-y-2 border-t md:border-t-0 md:border-l border-app pt-4 md:pt-0 md:pl-6">
            <div>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <span class="glow-dot" :class="settings.privacyTrackForegroundApp ? 'glow-dot-green' : 'glow-dot-muted'"></span>
                  <span class="text-sm font-semibold text-[var(--color-text)]">Foreground App Tracking</span>
                </div>
                <RouterLink to="/settings" class="text-[11px] text-[var(--color-accent)] hover:underline">
                  Configure
                </RouterLink>
              </div>
              <p class="text-xs text-muted mt-1">
                {{ settings.privacyTrackForegroundApp ? 'Active application detection is enabled' : 'Disabled for zero-telemetry privacy' }}
              </p>
            </div>
            <div class="text-[11px] text-muted">
              🔒 Local-only: No URLs, no keystrokes, no window titles.
            </div>
          </div>

        </div>
      </div>
    </div>

    <!-- ── Today's Work Sessions Timeline ──────────────────────────────── -->
    <div>
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted">{{ t('activity.workSessions') }}</h2>
        <RouterLink to="/history" class="text-xs text-[var(--color-accent)] hover:underline">
          {{ t('activity.viewHistory') }}
        </RouterLink>
      </div>

      <div v-if="statsStore.recentSessions.length === 0" class="card p-8 flex flex-col items-center text-center gap-2">
        <p class="text-sm font-medium text-[var(--color-text)]">{{ t('activity.noSessions') }}</p>
        <p class="text-xs text-muted">{{ t('activity.noSessionsHint') }}</p>
      </div>

      <div v-else class="space-y-2.5">
        <div
          v-for="session in statsStore.recentSessions.slice(0, 5)"
          :key="session.id"
          class="card-hover p-4 flex items-center justify-between gap-4"
        >
          <div class="flex items-center gap-3">
            <span class="w-2.5 h-2.5 rounded-full" :class="sessionDotClass(session.status)"></span>
            <div>
              <p class="text-sm font-medium text-[var(--color-text)]">
                {{ formatSessionTime(session.startedAtUtc) }}
              </p>
              <p class="text-xs text-muted">
                {{ formatDuration(session.activeSeconds) }} active · {{ formatDuration(session.idleSeconds) }} idle
              </p>
            </div>
          </div>
          <span class="badge" :class="sessionBadgeClass(session.status)">
            {{ session.status }}
          </span>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { RouterLink } from 'vue-router'
import { useActivityStore } from '@/stores/activity'
import { useStatisticsStore } from '@/stores/statistics'
import { useSettingsStore } from '@/stores/settings'
import { formatDuration } from '@/types'
import ActivityTimer from '@/components/ActivityTimer.vue'
import { useI18n } from 'vue-i18n'

const activityStore = useActivityStore()
const statsStore = useStatisticsStore()
const settingsStore = useSettingsStore()
const { t, locale } = useI18n()

const isRefreshing = ref(false)

// Active manual break state
interface ManualBreak {
  type: string
  duration: number
  title: string
  icon: string
  hint: string
}
const activeManualBreak = ref<ManualBreak | null>(null)
const breakCountdown = ref(0)
let breakTimerInterval: ReturnType<typeof setInterval> | null = null

onMounted(async () => {
  await activityStore.startListening()
  await statsStore.startListening()
  if (!settingsStore.settings) {
    await settingsStore.fetchSettings()
  }
})

onUnmounted(() => {
  if (breakTimerInterval) clearInterval(breakTimerInterval)
  activityStore.stopListening()
  statsStore.stopListening()
})

// ─── Computed
const capabilities = computed(() => activityStore.capabilities)
const settings = computed(() => settingsStore.settings ?? settingsStore.defaults)
const isPaused = computed(() => activityStore.isPaused || activityStore.state === 'Paused')
const isBreaking = computed(() => activityStore.state === 'Breaking')

const idleCutoffSec = computed(() => settings.value.activityIdleCutoffSeconds)
const shortBreakThresholdSec = computed(() => settings.value.breakShortAfterMinutes * 60)

const idleProgress = computed(() => {
  const cutoff = idleCutoffSec.value || 60
  return Math.min(100, (activityStore.idleSeconds / cutoff) * 100)
})

const stateBadgeClass = computed(() => {
  const s = activityStore.state
  if (s === 'Active')   return 'badge-success'
  if (s === 'Idle')     return 'badge-warning'
  if (s === 'BreakDue') return 'badge-danger'
  if (s === 'Breaking') return 'badge-success'
  return 'badge-muted'
})

const dotClass = computed(() => {
  const s = activityStore.state
  if (s === 'Active')   return 'glow-dot-green'
  if (s === 'Idle')     return 'glow-dot-amber'
  if (s === 'BreakDue') return 'glow-dot-red'
  if (s === 'Breaking') return 'glow-dot-green'
  return 'glow-dot-muted'
})

// ─── Methods
async function toggleMonitoring() {
  await activityStore.togglePause()
}

async function refreshData() {
  isRefreshing.value = true
  try {
    await Promise.all([
      activityStore.fetchActivity(),
      activityStore.fetchCapabilities(),
      statsStore.fetchToday(),
      statsStore.fetchRecentSessions(),
    ])
  } finally {
    setTimeout(() => {
      isRefreshing.value = false
    }, 400)
  }
}

function startManualBreak(type: string, durationSec: number, title: string, icon: string, hint: string) {
  activeManualBreak.value = {
    type,
    duration: durationSec,
    title,
    icon,
    hint,
  }
  breakCountdown.value = durationSec
  activityStore.triggerBreak(type)

  if (breakTimerInterval) clearInterval(breakTimerInterval)
  breakTimerInterval = setInterval(() => {
    if (breakCountdown.value > 0) {
      breakCountdown.value -= 1
    } else {
      finishActiveBreak()
    }
  }, 1000)
}

async function finishActiveBreak() {
  if (breakTimerInterval) clearInterval(breakTimerInterval)
  const duration = activeManualBreak.value ? activeManualBreak.value.duration - breakCountdown.value : 0
  const type = activeManualBreak.value?.type || 'manual'
  await activityStore.finishBreak(type, Math.max(1, duration))
  activeManualBreak.value = null
  breakCountdown.value = 0
}

async function snoozeActiveBreak() {
  if (breakTimerInterval) clearInterval(breakTimerInterval)
  const type = activeManualBreak.value?.type || 'short'
  await activityStore.snooze(type)
  activeManualBreak.value = null
  breakCountdown.value = 0
}

async function cancelActiveBreak() {
  if (breakTimerInterval) clearInterval(breakTimerInterval)
  const type = activeManualBreak.value?.type || 'manual'
  await activityStore.dismiss(type)
  activeManualBreak.value = null
  breakCountdown.value = 0
}

function formatSeconds(sec: number): string {
  const m = Math.floor(sec / 60)
  const s = sec % 60
  if (m === 0) return `${s}s`
  return `${m}:${String(s).padStart(2, '0')}`
}

function sessionDotClass(status: string) {
  if (status === 'running') return 'bg-green-500'
  if (status === 'completed') return 'bg-gray-500'
  return 'bg-amber-500'
}

function sessionBadgeClass(status: string) {
  if (status === 'running') return 'badge-success'
  if (status === 'completed') return 'badge-muted'
  return 'badge-warning'
}

function formatSessionTime(utc: string) {
  try {
    return new Intl.DateTimeFormat(locale.value === 'id' ? 'id-ID' : 'en-US', {
      hour: 'numeric',
      minute: '2-digit',
      hour12: true,
    }).format(new Date(utc))
  } catch {
    return utc
  }
}
</script>

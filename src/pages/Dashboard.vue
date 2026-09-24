<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">

    <!-- ── Header ─────────────────────────────────────────────────── -->
    <div class="flex items-start justify-between">
      <div>
        <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">Good morning</h1>
        <p class="text-[14px] text-muted mt-1">Here's your desk activity today.</p>
        <p class="text-[14px] text-muted mt-2">{{ currentDate }}</p>
      </div>
      <div class="flex flex-col items-end gap-4">
        <button class="btn-primary" @click="router.push('/activity')">Take a Break</button>
        <div class="flex items-center gap-2">
          <span class="glow-dot glow-dot-green"></span>
          <span class="text-[13px] font-medium text-[var(--color-text)]">Monitoring</span>
        </div>
      </div>
    </div>

    <!-- ── Loading state ──────────────────────────────────────────── -->
    <div v-if="activityStore.loading" class="flex items-center justify-center py-20">
      <div class="flex flex-col items-center gap-4">
        <div class="w-8 h-8 rounded-full border-2 border-[var(--color-accent)] border-t-transparent animate-spin"></div>
        <span class="text-sm text-muted">Connecting to monitor…</span>
      </div>
    </div>

    <template v-else>
      <!-- ── AI Tip ─────────────────────────────────────────────────── -->
      <div v-if="aiTip" class="card bg-[var(--color-surface)] border-[var(--color-accent)] border p-4 flex items-start gap-4 animate-fade-in shadow-sm shadow-[var(--color-accent)]/10">
        <div class="text-[20px]">✨</div>
        <div>
          <h3 class="text-xs font-semibold uppercase tracking-widest text-[var(--color-accent)] mb-1">Your AI Coach</h3>
          <p class="text-sm text-[var(--color-text)] leading-relaxed">{{ aiTip }}</p>
        </div>
      </div>

      <!-- ── Current Session ───────────────────────────────────────── -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

        <!-- Activity ring timer -->
        <ActivityTimer
          :state="activityStore.state"
          :seconds="activityStore.activeSeconds"
          :max-seconds="shortBreakThresholdSec"
          subtitle="active session"
        />

        <!-- Idle + state info -->
        <div class="card p-6 flex flex-col justify-between gap-4">
          <!-- State badge -->
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold uppercase tracking-widest text-muted">Current State</span>
            <span class="badge" :class="stateBadgeClass">
              <span class="glow-dot text-[10px]" :class="dotClass"></span>
              {{ activityStore.stateText }}
            </span>
          </div>

          <!-- Idle counter -->
          <div class="flex flex-col gap-1">
            <span class="text-xs font-semibold uppercase tracking-widest text-muted">Idle Time</span>
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
              {{ idleProgress >= 100 ? 'Break auto-detected' : `${idleCutoffSec}s until auto-break` }}
            </span>
          </div>

          <!-- Platform capability detail -->
          <div v-if="capabilities" class="border-t border-app pt-4">
            <div class="grid grid-cols-2 gap-y-2 gap-x-4 mt-2">
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities.idleDetection ? 'glow-dot-green' : 'glow-dot-amber'"></span>
                <span class="text-[12px] text-muted">Idle Detection</span>
              </div>
              <div class="text-[12px] font-medium" :class="capabilities.idleDetection ? 'text-green-500' : 'text-amber-500'">
                {{ capabilities.idleDetection ? 'Available' : 'Fallback' }}
              </div>
              <div class="flex items-center gap-2">
                <span class="glow-dot" :class="capabilities.sessionLockDetection ? 'glow-dot-green' : 'glow-dot-muted'"></span>
                <span class="text-[12px] text-muted">Screen Lock</span>
              </div>
              <div class="text-[12px] font-medium" :class="capabilities.sessionLockDetection ? 'text-green-500' : 'text-muted'">
                {{ capabilities.sessionLockDetection ? 'Available' : 'Unavailable' }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- ── Today Stats ────────────────────────────────────────────── -->
      <div>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">Today's Overview</h2>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <StatCard
            label="Active Time"
            :value="formatDuration(todayStats?.activeSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="0"
          />
          <StatCard
            label="Break Time"
            :value="formatDuration(todayStats?.breakSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="1"
          />
          <StatCard
            label="Breaks Taken"
            :value="String(todayStats?.breakCount ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="2"
          />
          <StatCard
            label="Longest Streak"
            :value="formatDuration(todayStats?.longestActiveStreakSeconds ?? 0)"
            icon=""
            icon-bg-class=""
            :delay="3"
          />
        </div>
      </div>

      <!-- ── Break Schedule ────────────────────────────────────────── -->
      <div>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">Next Break</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <BreakCard
            title="Eye Rest"
            :description="`In ${activityStore.activity?.nextEyeBreakSeconds ?? 0}s`"
            icon=""
            icon-bg=""
            :countdown-seconds="activityStore.activity?.nextEyeBreakSeconds ?? null"
            :progress="eyeProgress"
            progress-color="var(--color-blue)"
          />
          <BreakCard
            title="Movement Break"
            :description="`In ${activityStore.activity?.nextShortBreakSeconds ?? 0}s`"
            icon=""
            icon-bg=""
            :countdown-seconds="activityStore.activity?.nextShortBreakSeconds ?? null"
            :progress="shortProgress"
            progress-color="var(--color-green)"
          />
          <BreakCard
            title="Long Break"
            :description="`In ${activityStore.activity?.nextLongBreakSeconds ?? 0}s`"
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
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const activityStore = useActivityStore()
const statsStore = useStatisticsStore()
const settingsStore = useSettingsStore()

const aiTip = ref<string | null>(null)

// ─── Lifecycle
onMounted(async () => {
  await activityStore.startListening()
  await statsStore.startListening()
  if (!settingsStore.settings) await settingsStore.fetchSettings()
  
  if (settingsStore.settings?.aiEnabled) {
    try {
      aiTip.value = await invoke<string | null>('generate_daily_tip')
    } catch (e) {
      console.error("AI tip failed:", e)
    }
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
  new Intl.DateTimeFormat('en-US', { weekday: 'long', month: 'long', day: 'numeric' }).format(new Date())
)
</script>

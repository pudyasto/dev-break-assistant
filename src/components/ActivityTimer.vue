<template>
  <div class="card p-6 flex flex-col items-center justify-center gap-4">
    <!-- Ring timer -->
    <div class="relative" style="width: 160px; height: 160px;">
      <svg class="absolute inset-0 -rotate-90" width="160" height="160" viewBox="0 0 160 160">
        <!-- Track -->
        <circle
          cx="80" cy="80" r="70"
          fill="none"
          stroke="var(--color-border)"
          stroke-width="8"
        />
        <!-- Progress arc -->
        <circle
          cx="80" cy="80" r="70"
          fill="none"
          :stroke="arcColor"
          stroke-width="8"
          stroke-linecap="round"
          :stroke-dasharray="circumference"
          :stroke-dashoffset="dashOffset"
          style="transition: stroke-dashoffset 1s ease, stroke 0.5s ease;"
        />
      </svg>

      <!-- Center content -->
      <div class="absolute inset-0 flex flex-col items-center justify-center gap-1">
        <!-- Animated state dot -->
        <div class="flex items-center gap-1.5 mb-1">
          <span class="glow-dot" :class="dotClass" style="animation: pulse 2s infinite;"></span>
          <span class="text-[11px] font-semibold uppercase tracking-widest" :class="textClass">
            {{ stateLabel }}
          </span>
        </div>
        <!-- Main time -->
        <span class="text-3xl font-bold font-mono leading-none text-[var(--color-text)] animate-counter">
          {{ formattedTime }}
        </span>
        <span class="text-[12px] text-muted">{{ subtitle }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ActivityState } from '@/types'
import { stateLabel as getStateLabel } from '@/types'

interface Props {
  state: ActivityState
  seconds: number
  /** Max seconds for the ring progress (e.g. next break threshold) */
  maxSeconds?: number
  subtitle?: string
}

const props = withDefaults(defineProps<Props>(), {
  maxSeconds: 2700, // 45 min default
  subtitle: 'active session',
})

const circumference = 2 * Math.PI * 70 // r=70

const progress = computed(() => {
  if (!props.maxSeconds) return 0
  return Math.min(1, props.seconds / props.maxSeconds)
})

const dashOffset = computed(() => circumference - progress.value * circumference)

const stateLabel = computed(() => getStateLabel(props.state))

const arcColor = computed(() => {
  const s = props.state
  if (s === 'Active')       return 'var(--color-accent)'
  if (s === 'Idle')         return 'var(--color-amber)'
  if (s === 'BreakDue')     return 'var(--color-red)'
  if (s === 'Breaking')     return 'var(--color-green)'
  return 'var(--color-muted)'
})

const dotClass = computed(() => {
  const s = props.state
  if (s === 'Active')   return 'glow-dot-green'
  if (s === 'Idle')     return 'glow-dot-amber'
  if (s === 'BreakDue') return 'glow-dot-red'
  if (s === 'Breaking') return 'glow-dot-green'
  return 'glow-dot-muted'
})

const textClass = computed(() => {
  const s = props.state
  if (s === 'Active')   return 'text-green-500'
  if (s === 'Idle')     return 'text-amber-500'
  if (s === 'BreakDue') return 'text-red-500'
  if (s === 'Breaking') return 'text-blue-500'
  return 'text-muted'
})

const formattedTime = computed(() => {
  const s = props.seconds
  const m = Math.floor(s / 60)
  const sec = s % 60
  if (m < 60) {
    return `${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
  }
  const h = Math.floor(m / 60)
  const rm = m % 60
  return `${h}:${String(rm).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
})
</script>

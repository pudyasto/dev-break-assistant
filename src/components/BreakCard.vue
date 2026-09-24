<template>
  <div class="card p-5 flex flex-col justify-between gap-3">
    <div class="flex items-center justify-between gap-2">
      <span class="font-semibold text-[15px] text-[var(--color-text)]">{{ title }}</span>
      <span v-if="countdownSeconds !== null" class="font-mono text-[13px] font-medium" :class="countdownClass">
        {{ formattedCountdown }}
      </span>
    </div>
    <p class="text-[13px] text-muted">{{ description }}</p>
    <!-- Progress -->
    <div v-if="progress !== null" class="progress-bar mt-1">
      <div class="progress-fill" :style="{ width: `${Math.min(100, progress)}%`, background: progressColor }"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  title: string
  description: string
  countdownSeconds?: number | null
  /** Progress 0–100 */
  progress?: number | null
  progressColor?: string
}

const props = withDefaults(defineProps<Props>(), {
  countdownSeconds: null,
  progress: null,
  progressColor: 'var(--color-accent)',
})

const countdownClass = computed(() => {
  if (props.countdownSeconds === null) return ''
  if (props.countdownSeconds <= 60)  return 'text-red-400'
  if (props.countdownSeconds <= 300) return 'text-amber-400'
  return 'text-[var(--color-text)]'
})

const formattedCountdown = computed(() => {
  if (props.countdownSeconds === null) return ''
  const s = props.countdownSeconds
  const m = Math.floor(s / 60)
  const sec = s % 60
  return m > 0 ? `${m}m ${sec}s` : `${sec}s`
})
</script>

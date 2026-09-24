<template>
  <div class="stat-card animate-slide-up" :style="{ animationDelay: `${delay * 80}ms` }">
    <div class="flex flex-col gap-1 min-w-0">
      <span class="text-xs font-semibold uppercase tracking-widest text-muted truncate">{{ label }}</span>
      <span class="text-3xl font-bold tracking-tight mt-1" :class="valueClass">
        {{ value }}
      </span>
      <span v-if="subtitle" class="text-[13px] text-muted mt-1">{{ subtitle }}</span>
    </div>
    <!-- Optional progress bar -->
    <div v-if="progress !== undefined && progress !== null" class="progress-bar mt-3">
      <div
        class="progress-fill"
        :style="{ width: `${Math.min(100, Math.max(0, progress))}%` }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  label: string
  value: string
  subtitle?: string
  valueClass?: string
  progress?: number | null
  delay?: number
}

withDefaults(defineProps<Props>(), {
  valueClass: 'text-[var(--color-text)]',
  delay: 0,
})
</script>

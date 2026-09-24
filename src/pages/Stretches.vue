<template>
  <div class="h-full flex flex-col p-8 bg-[var(--color-bg)] animate-fade-in">
    <div class="mb-8">
      <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">Stretch Library</h1>
      <p class="text-[14px] text-muted mt-1">Simple and safe desk stretches to relieve tension.</p>
    </div>

    <div class="flex-1 overflow-y-auto pr-2">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div 
          v-for="stretch in STRETCH_LIBRARY" 
          :key="stretch.id" 
          class="card p-6 flex flex-col hover:border-[var(--color-border-hover)] transition-colors group cursor-pointer"
        >
          <div class="flex items-start justify-between mb-4">
            <h3 class="text-lg font-medium text-[var(--color-text)] group-hover:text-[var(--color-accent)] transition-colors">
              {{ stretch.name }}
            </h3>
            <span class="text-xs font-mono px-2 py-1 bg-[var(--color-hover)] text-muted rounded">
              {{ stretch.durationSeconds }}s
            </span>
          </div>
          
          <p class="text-[14px] text-[var(--color-text-muted)] leading-relaxed flex-1 mb-4">
            {{ stretch.description }}
          </p>

          <div class="mt-auto pt-4 border-t border-[var(--color-border)]">
            <p class="text-[12px] font-medium text-emerald-500/80 uppercase tracking-wider mb-1">Benefits</p>
            <p class="text-[13px] text-muted">
              {{ stretch.benefits }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { STRETCH_LIBRARY } from '@/data/stretches'
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-[var(--color-bg)]">

    <!-- ── Sidebar ────────────────────────────────────────────────── -->
    <aside class="flex flex-col flex-shrink-0 border-r border-app bg-surface" style="width: var(--sidebar-w);">

      <!-- Header -->
      <div class="flex items-center gap-3 px-5 py-5 border-b border-app">
        <div class="w-6 h-6 rounded flex items-center justify-center text-white bg-[var(--color-accent)]">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
          </svg>
        </div>
        <div>
          <p class="font-semibold text-[13px] text-[var(--color-text)] leading-tight tracking-wide">DevBreak</p>
        </div>
      </div>

      <!-- Nav links -->
      <nav class="flex-1 px-3 py-4 space-y-1">
        <RouterLink
          v-for="item in navItems"
          :key="item.to"
          :to="item.to"
          class="nav-item group"
          :class="{ active: route.path === item.to }"
        >
          <span class="w-5 h-5 flex items-center justify-center" v-html="item.icon"></span>
          <span>{{ item.label }}</span>
        </RouterLink>
      </nav>

      <!-- Bottom status pill -->
      <div class="px-3 py-4 border-t border-app">
        <div class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg border border-app bg-surface hover:bg-[var(--color-hover)] transition-colors cursor-pointer">
          <span class="glow-dot" :class="statusDotClass"></span>
          <div class="min-w-0">
            <p class="text-[13px] font-medium text-[var(--color-text)] leading-tight truncate">{{ statusText }}</p>
            <p class="text-[11px] text-muted leading-tight truncate">Ubuntu 24.04</p>
          </div>
        </div>
      </div>
    </aside>

    <!-- ── Main content ───────────────────────────────────────────── -->
    <main class="flex-1 min-w-0 overflow-hidden bg-[var(--color-bg)]">
      <RouterView v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </main>

  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink, RouterView, useRoute } from 'vue-router'
import { useActivityStore } from '@/stores/activity'

const route = useRoute()
const activityStore = useActivityStore()

const iconDashboard = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="9" rx="1"></rect><rect x="14" y="3" width="7" height="5" rx="1"></rect><rect x="14" y="12" width="7" height="9" rx="1"></rect><rect x="3" y="16" width="7" height="5" rx="1"></rect></svg>`
const iconActivity = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline></svg>`
const iconHistory = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>`
const iconInsights = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="20" x2="18" y2="10"></line><line x1="12" y1="20" x2="12" y2="4"></line><line x1="6" y1="20" x2="6" y2="14"></line></svg>`
const iconStretches = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path></svg>`
const iconSettings = `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>`

const navItems = [
  { to: '/',           icon: iconDashboard, label: 'Dashboard' },
  { to: '/activity',   icon: iconActivity,  label: 'Activity'  },
  { to: '/history',    icon: iconHistory,   label: 'History'   },
  { to: '/statistics', icon: iconInsights,  label: 'Insights'  },
  { to: '/stretches',  icon: iconStretches, label: 'Stretches' },
  { to: '/settings',   icon: iconSettings,  label: 'Settings'  },
]

const statusDotClass = computed(() => {
  const s = activityStore.state
  if (s === 'Active')   return 'glow-dot-green'
  if (s === 'Idle')     return 'glow-dot-amber'
  if (s === 'BreakDue') return 'glow-dot-red'
  return 'glow-dot-muted'
})

const statusText = computed(() => {
  if (activityStore.state === 'Idle') return 'Idle'
  if (activityStore.state === 'BreakDue') return 'Break Due'
  return 'Monitoring'
})
</script>

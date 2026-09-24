<template>
  <div class="h-full overflow-y-auto p-8 space-y-8 animate-fade-in bg-[var(--color-bg)]">

    <div>
      <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('settings.title') }}</h1>
      <p class="text-[14px] text-muted mt-1">{{ t('settings.subtitle') }}</p>
    </div>

    <div v-if="settingsStore.loading" class="flex items-center justify-center py-20">
      <div class="w-8 h-8 rounded-full border-2 border-[var(--color-accent)] border-t-transparent animate-spin"></div>
    </div>

    <template v-else-if="form">
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('language.title') }}</h2>
        <div class="card">
          <SettingRow :label="t('language.title')" :description="t('language.description')">
            <select class="form-input text-sm" :value="locale" @change="changeLanguage(($event.target as HTMLSelectElement).value)">
              <option value="id">{{ t('language.indonesian') }}</option>
              <option value="en">{{ t('language.english') }}</option>
            </select>
          </SettingRow>
        </div>
      </section>

      <!-- ── Break Intervals ─────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.breakIntervals') }}</h2>
        <div class="card divide-y divide-app">

          <SettingRow
            :label="t('settings.eyeBreak')"
            :description="t('settings.eyeBreakDesc')"
          >
            <NumberInput v-model="form.breakEyeAfterMinutes" :min="5" :max="60" suffix="min" id="setting-eye-after" />
          </SettingRow>

          <SettingRow
            :label="t('settings.shortBreak')"
            :description="t('settings.shortBreakDesc')"
          >
            <NumberInput v-model="form.breakShortAfterMinutes" :min="15" :max="120" suffix="min" id="setting-short-after" />
          </SettingRow>

          <SettingRow
            :label="t('settings.longBreak')"
            :description="t('settings.longBreakDesc')"
          >
            <NumberInput v-model="form.breakLongAfterMinutes" :min="60" :max="240" suffix="min" id="setting-long-after" />
          </SettingRow>
        </div>
      </section>

      <!-- ── Break Durations ────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.breakDurations') }}</h2>
        <div class="card divide-y divide-app">

          <SettingRow :label="t('settings.eyeDuration')">
            <NumberInput v-model="form.breakEyeDurationSeconds" :min="10" :max="60" suffix="sec" id="setting-eye-dur" />
          </SettingRow>

          <SettingRow :label="t('settings.shortDuration')">
            <NumberInput v-model="form.breakShortDurationSeconds" :min="60" :max="600" suffix="sec" id="setting-short-dur" />
          </SettingRow>

          <SettingRow :label="t('settings.longDuration')">
            <NumberInput v-model="form.breakLongDurationSeconds" :min="120" :max="1800" suffix="sec" id="setting-long-dur" />
          </SettingRow>

        </div>
      </section>

      <!-- ── Detection ─────────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.detection') }}</h2>
        <div class="card divide-y divide-app">

          <SettingRow
            :label="t('settings.idleCutoff')"
            :description="t('settings.idleCutoffDesc')"
          >
            <NumberInput v-model="form.activityIdleCutoffSeconds" :min="10" :max="300" suffix="sec" id="setting-idle-cutoff" />
          </SettingRow>

          <SettingRow
            :label="t('settings.autoBreak')"
            :description="t('settings.autoBreakDesc')"
          >
            <NumberInput v-model="form.breakAutoCompleteIdleSeconds" :min="60" :max="600" suffix="sec" id="setting-auto-break" />
          </SettingRow>

        </div>
      </section>

      <!-- ── App Behavior ──────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.behavior') }}</h2>
        <div class="card divide-y divide-app">

          <SettingRow :label="t('settings.notifications')">
            <ToggleInput v-model="form.notificationEnabled" id="setting-notifications" />
          </SettingRow>

          <SettingRow :label="t('settings.startLogin')">
            <ToggleInput v-model="form.autostartEnabled" id="setting-autostart" />
          </SettingRow>

          <SettingRow :label="t('settings.startMinimized')">
            <ToggleInput v-model="form.uiStartMinimized" id="setting-start-minimized" />
          </SettingRow>

        </div>
      </section>

      <!-- ── Privacy ───────────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.privacy') }}</h2>
        <div class="card divide-y divide-app">

          <SettingRow
            :label="t('settings.trackApp')"
            :description="t('settings.trackAppDesc')"
          >
            <ToggleInput v-model="form.privacyTrackForegroundApp" id="setting-track-app" />
          </SettingRow>

        </div>
      </section>

      <!-- ── AI Integration ────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-muted mb-4">{{ t('settings.ai') }}</h2>
        <div class="card divide-y divide-app">
          <SettingRow :label="t('settings.aiEnable')" :description="t('settings.aiEnableDesc')">
            <ToggleInput v-model="form.aiEnabled" id="setting-ai-enabled" />
          </SettingRow>
          <SettingRow :label="t('settings.aiEndpoint')" :description="t('settings.aiEndpointDesc')">
            <StringInput v-model="form.aiEndpoint" class="w-64 sm:w-80" id="setting-ai-endpoint" />
          </SettingRow>
          <SettingRow :label="t('settings.aiModel')" :description="t('settings.aiModelDesc')">
            <StringInput v-model="form.aiModel" class="w-64 sm:w-80" id="setting-ai-model" />
          </SettingRow>

          <div class="px-6 py-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 bg-[var(--color-bg-secondary)]/30">
            <div class="min-w-0">
              <p class="text-[13px] font-medium text-[var(--color-text)]">{{ t('settings.testTitle') }}</p>
              <p v-if="testResult" class="text-xs mt-1" :class="testResult.success ? 'text-green-400' : 'text-red-400'">
                {{ testResult.message }}
              </p>
              <p v-else class="text-xs text-muted mt-0.5">
                {{ t('settings.testHint') }}
              </p>
            </div>
            <button
              id="test-ai-btn"
              type="button"
              class="btn-secondary text-xs px-3 py-2 flex items-center gap-2 self-start sm:self-auto shrink-0"
              :disabled="testingAi || !form.aiEndpoint"
              @click="handleTestAi"
            >
              <span v-if="testingAi" class="w-3.5 h-3.5 rounded-full border-2 border-current border-t-transparent animate-spin"></span>
              {{ testingAi ? t('settings.testing') : t('settings.test') }}
            </button>
          </div>
        </div>
      </section>

      <!-- ── Data Management ────────────────────────────────────── -->
      <section>
        <h2 class="text-xs font-semibold uppercase tracking-widest text-red-400 mb-4">{{ t('settings.data') }}</h2>
        <div class="card border border-red-500/20 divide-y divide-app bg-red-950/10">
          <div class="flex items-center justify-between px-6 py-5 gap-4">
            <div class="min-w-0">
              <p class="text-[14px] font-medium text-[var(--color-text)]">{{ t('settings.resetTitle') }}</p>
              <p class="text-[13px] text-muted mt-1 leading-relaxed">
                {{ t('settings.resetDesc') }}
              </p>
            </div>
            <button
              id="reset-data-btn"
              type="button"
              class="btn-danger whitespace-nowrap shrink-0 flex items-center gap-2"
              @click="showResetModal = true"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              {{ t('settings.reset') }}
            </button>
          </div>
        </div>
      </section>

      <!-- ── Save button ────────────────────────────────────────── -->
      <div class="flex items-center justify-between pt-2">
        <div class="flex items-center gap-4">
          <span v-if="saved" class="flex items-center gap-2 text-sm text-green-500 animate-fade-in">
            <span>✓</span> {{ t('settings.saved') }}
          </span>
          <span v-if="resetSuccess" class="flex items-center gap-2 text-sm text-green-500 animate-fade-in">
            <span>✓</span> {{ t('settings.resetSuccess') }}
          </span>
          <span v-else-if="settingsStore.error" class="text-sm text-red-500">{{ settingsStore.error }}</span>
        </div>

        <button
          id="settings-save-btn"
          class="btn-primary"
          :disabled="settingsStore.saving"
          @click="save"
        >
          <span v-if="settingsStore.saving" class="w-4 h-4 rounded-full border-2 border-[var(--color-bg)] border-t-transparent animate-spin"></span>
          {{ settingsStore.saving ? t('common.saving') : t('common.save') }}
        </button>
      </div>

    </template>

    <!-- ── Reset Confirmation Modal ─────────────────────────────── -->
    <div
      v-if="showResetModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-sm animate-fade-in"
    >
      <div class="card w-full max-w-md p-6 space-y-4 border border-red-500/30 shadow-2xl bg-[var(--color-surface)]">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-full bg-red-500/10 text-red-400 shrink-0">
            <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <div>
            <h3 class="text-base font-semibold text-[var(--color-text)]">{{ t('settings.confirmTitle') }}</h3>
            <p class="text-xs text-muted">{{ t('settings.confirmWarning') }}</p>
          </div>
        </div>

        <p class="text-sm text-[var(--color-text-sec)] leading-relaxed">
          {{ t('settings.confirmText') }}
        </p>

        <div class="flex items-center justify-end gap-3 pt-2">
          <button
            type="button"
            class="btn-ghost text-sm"
            :disabled="resetting"
            @click="showResetModal = false"
          >
            {{ t('common.cancel') }}
          </button>
          <button
            id="confirm-reset-btn"
            type="button"
            class="btn bg-red-600 hover:bg-red-700 text-white text-sm flex items-center gap-2"
            :disabled="resetting"
            @click="confirmResetData"
          >
            <span v-if="resetting" class="w-4 h-4 rounded-full border-2 border-white border-t-transparent animate-spin"></span>
            {{ resetting ? t('settings.resetting') : t('settings.deleteAll') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useStatisticsStore } from '@/stores/statistics'
import { useActivityStore } from '@/stores/activity'
import { resetAllData, testAiConnection } from '@/services/tauri'
import type { AppSettings } from '@/types'
import { useI18n } from 'vue-i18n'
import { setLocale, type SupportedLocale } from '@/i18n'

// ─── Sub-components defined inline for simplicity
import { defineComponent, h } from 'vue'

const SettingRow = defineComponent({
  props: {
    label: String,
    description: String,
  },
  setup(props, { slots }) {
    return () => h('div', { class: 'flex items-center justify-between px-6 py-5 gap-4 hover:bg-[var(--color-hover)] transition-colors' }, [
      h('div', { class: 'flex items-center gap-3 min-w-0' }, [
        h('div', { class: 'min-w-0' }, [
          h('p', { class: 'text-[14px] font-medium text-[var(--color-text)]' }, props.label),
          props.description ? h('p', { class: 'text-[13px] text-muted mt-1 leading-relaxed' }, props.description) : null,
        ]),
      ]),
      h('div', { class: 'flex-shrink-0' }, slots.default?.()),
    ])
  },
})

const NumberInput = defineComponent({
  props: { modelValue: Number, min: Number, max: Number, suffix: String, id: String },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () => h('div', { class: 'flex items-center gap-2' }, [
      h('input', {
        id: props.id,
        type: 'number',
        value: props.modelValue,
        min: props.min,
        max: props.max,
        class: 'form-input w-24 text-center font-mono',
        onInput: (e: Event) => emit('update:modelValue', Number((e.target as HTMLInputElement).value)),
      }),
      props.suffix ? h('span', { class: 'text-xs text-muted w-8' }, props.suffix) : null,
    ])
  },
})

const ToggleInput = defineComponent({
  props: { modelValue: Boolean, id: String },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () => h('label', { class: 'toggle', for: props.id }, [
      h('input', {
        id: props.id,
        type: 'checkbox',
        checked: props.modelValue,
        onChange: (e: Event) => emit('update:modelValue', (e.target as HTMLInputElement).checked),
      }),
      h('span', { class: 'toggle-slider' }),
    ])
  },
})

const StringInput = defineComponent({
  props: { modelValue: String, id: String, class: String },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () => h('input', {
      id: props.id,
      type: 'text',
      value: props.modelValue,
      class: ['form-input text-sm', props.class || 'w-64'].join(' '),
      onInput: (e: Event) => emit('update:modelValue', (e.target as HTMLInputElement).value),
    })
  },
})

// ─── Page logic
const settingsStore = useSettingsStore()
const statisticsStore = useStatisticsStore()
const activityStore = useActivityStore()
const { t, locale } = useI18n()

const form = ref<AppSettings | null>(null)
const saved = ref(false)
const showResetModal = ref(false)
const resetting = ref(false)
const resetSuccess = ref(false)

const testingAi = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)

async function handleTestAi() {
  if (!form.value?.aiEndpoint) return
  testingAi.value = true
  testResult.value = null
  try {
    const res = await testAiConnection(form.value.aiEndpoint, form.value.aiModel || 'llama3')
    testResult.value = {
      success: true,
      message: t('settings.testSuccess', { response: `${res.slice(0, 80)}${res.length > 80 ? '...' : ''}` })
    }
  } catch (err: any) {
    testResult.value = {
      success: false,
      message: t('settings.testError', { error: err?.message || err })
    }
  } finally {
    testingAi.value = false
  }
}

function changeLanguage(value: string) {
  if (value === 'en' || value === 'id') setLocale(value as SupportedLocale)
}

onMounted(async () => {
  await settingsStore.fetchSettings()
  if (settingsStore.settings) {
    form.value = { ...settingsStore.settings }
  }
})

watch(() => settingsStore.settings, (s) => {
  if (s && !form.value) form.value = { ...s }
})

async function save() {
  if (!form.value) return
  try {
    await settingsStore.saveSettings(form.value)
    saved.value = true
    setTimeout(() => { saved.value = false }, 3000)
  } catch {
    // error shown by store
  }
}

async function confirmResetData() {
  resetting.value = true
  try {
    await resetAllData()
    showResetModal.value = false
    resetSuccess.value = true
    await statisticsStore.fetchToday()
    await statisticsStore.fetchRecentSessions()
    await activityStore.fetchActivity()
    setTimeout(() => {
      resetSuccess.value = false
    }, 4000)
  } catch (err) {
    console.error('Failed to reset data:', err)
  } finally {
    resetting.value = false
  }
}
</script>

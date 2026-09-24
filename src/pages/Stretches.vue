<template>
  <div class="h-full flex flex-col p-8 bg-[var(--color-bg)] animate-fade-in">
    <div class="mb-8">
      <h1 class="text-[26px] font-semibold text-[var(--color-text)] tracking-tight">{{ t('stretches.title') }}</h1>
      <p class="text-[14px] text-muted mt-1">{{ t('stretches.subtitle') }}</p>
    </div>

    <div class="flex-1 overflow-y-auto pr-2">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div 
          v-for="stretch in stretches"
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
            <p class="text-[12px] font-medium text-emerald-500/80 uppercase tracking-wider mb-1">{{ t('stretches.benefits') }}</p>
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
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

const indonesianStretches: Record<string, { name: string; description: string; benefits: string }> = {
  'neck-stretch': {
    name: 'Peregangan leher',
    description: 'Miringkan kepala perlahan ke arah bahu sambil menahan lengan sisi sebaliknya. Ulangi pada sisi lain.',
    benefits: 'Meredakan ketegangan leher akibat menatap layar.',
  },
  'wrist-extension': {
    name: 'Peregangan pergelangan tangan',
    description: 'Luruskan lengan ke depan dengan telapak menghadap atas. Gunakan tangan lain untuk menarik jari perlahan ke arah lantai.',
    benefits: 'Membantu mencegah cedera akibat gerakan berulang pada pergelangan tangan.',
  },
  'seated-twist': {
    name: 'Putaran duduk',
    description: 'Saat duduk, letakkan tangan kanan di lutut kiri dan putar tubuh perlahan ke kiri. Ulangi pada sisi lain.',
    benefits: 'Meningkatkan kelenturan tulang belakang dan meredakan kekakuan punggung bawah.',
  },
  'shoulder-shrug': {
    name: 'Angkat bahu',
    description: 'Angkat kedua bahu ke arah telinga, tahan 5 detik, lalu putar ke belakang dan turunkan.',
    benefits: 'Melepaskan ketegangan di punggung atas dan bahu.',
  },
  'eye-palming': {
    name: 'Relaksasi mata dengan telapak tangan',
    description: 'Gosok kedua telapak tangan hingga hangat, lalu tutup mata yang terpejam dengan telapak tangan secara perlahan.',
    benefits: 'Merilekskan otot mata dan mengurangi ketegangan mata digital.',
  },
}

const stretches = computed(() => STRETCH_LIBRARY.map((stretch) => {
  const translation = locale.value === 'id' ? indonesianStretches[stretch.id] : undefined
  return translation ? { ...stretch, ...translation } : stretch
}))
</script>

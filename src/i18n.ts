import { createI18n } from 'vue-i18n'

import en from './locales/en.json'
import id from './locales/id.json'

export type SupportedLocale = 'en' | 'id'

const storedLocale = localStorage.getItem('language')
const browserLocale = navigator.language.toLowerCase().startsWith('id') ? 'id' : 'en'
const defaultLocale: SupportedLocale = storedLocale === 'id' || storedLocale === 'en'
  ? storedLocale
  : browserLocale

const i18n = createI18n({
  legacy: false, // Use Composition API
  locale: defaultLocale,
  fallbackLocale: 'en',
  messages: {
    en,
    id
  }
})

export default i18n

export function setLocale(locale: SupportedLocale) {
  i18n.global.locale.value = locale
  localStorage.setItem('language', locale)
}

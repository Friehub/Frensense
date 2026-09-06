// SAFE: Pinia store used for sensitive data, provide/inject only for UI state

import { provide, inject, ref, type Ref } from 'vue'

const THEME_KEY = Symbol('theme')

export function useThemeProvider(theme: Ref<'light' | 'dark'>) {
  provide(THEME_KEY, theme)
}

export function useTheme(): Ref<'light' | 'dark'> {
  const theme = inject<Ref<'light' | 'dark'>>(THEME_KEY)
  if (!theme) return ref('light') as Ref<'light' | 'dark'>
  return theme
}

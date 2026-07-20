// [frensense]
// observation: Context provider value is a new object reference every render, causing all consumers to re-render even when data hasn't changed
// impact: performance denial-of-service (perf DoS) — large component trees re-render on every keystroke or state change
// improvement: memoize the context value with `useMemo` to provide a stable reference

'use client'

import { createContext, useContext, useMemo, useState } from 'react'
import type { ReactNode } from 'react'

interface ThemeContextValue {
  theme: string
  setTheme: (t: string) => void
}

const ThemeContext = createContext<ThemeContextValue | null>(null)

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setTheme] = useState('light')

  // SAFE: memoized object prevents unnecessary consumer re-renders
  const value = useMemo(() => ({ theme, setTheme }), [theme])

  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  )
}

export function useTheme() {
  const ctx = useContext(ThemeContext)
  if (!ctx) throw new Error('missing provider')
  return ctx
}

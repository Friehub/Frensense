// SAFE: stores sensitive data in memory only; only non-sensitive UI prefs are persisted

import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface SessionStore {
  accessToken: string
  theme: string
  setTokens: (access: string) => void
}

export const useSessionStore = create<SessionStore>()(
  persist(
    (set) => ({
      accessToken: '',
      theme: 'light',
      setTokens: (access) => set({ accessToken: access }),
    }),
    {
      name: 'ui-prefs',
      partialize: (state) => ({ theme: state.theme }),
      merge: (persisted, current) => ({ ...current, ...persisted }),
    },
  ),
)

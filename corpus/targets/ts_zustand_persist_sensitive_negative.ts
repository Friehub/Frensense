// SAFE: partialize omits sensitive fields from persistence

import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface SessionStore {
  accessToken: string
  refreshToken: string
  theme: string
  setTokens: (access: string, refresh: string) => void
}

export const useSessionStore = create<SessionStore>()(
  persist(
    (set) => ({
      accessToken: '',
      refreshToken: '',
      theme: 'light',
      setTokens: (access, refresh) => set({ accessToken: access, refreshToken: refresh }),
    }),
    {
      name: 'session-storage',
      partialize: (state) => ({ theme: state.theme }),
    },
  ),
)

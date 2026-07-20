// [frensense]
// observation: Zustand persist middleware stores sensitive data (tokens, PII) in localStorage
// impact: sensitive data persisted in cleartext; accessible via XSS or physical access
// improvement: exclude sensitive fields from persist, or use sessionStorage with encryption

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
    { name: 'session-storage' },
  ),
)

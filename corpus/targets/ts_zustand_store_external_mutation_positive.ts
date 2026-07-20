// [frensense]
// observation: Zustand store state is mutated directly outside of `set()`, bypassing reactivity
// impact: React components do not re-render; state becomes inconsistent
// improvement: always use the store's `set()` method to update state

import { create } from 'zustand'

interface AuthStore {
  token: string | null
  user: { name: string } | null
}

export const useAuthStore = create<AuthStore>(() => ({
  token: null,
  user: null,
}))

export function login(token: string, name: string) {
  useAuthStore.getState().token = token
  useAuthStore.getState().user = { name }
}

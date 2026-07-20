// SAFE: external logic uses set() via the store API, not direct mutation

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
  useAuthStore.setState({ token, user: { name } })
}

export function logout() {
  useAuthStore.setState({ token: null, user: null })
}

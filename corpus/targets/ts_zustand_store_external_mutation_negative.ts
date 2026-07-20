// SAFE: updates state through the store's set function

import { create } from 'zustand'

interface AuthStore {
  token: string | null
  user: { name: string } | null
  login: (token: string, name: string) => void
  logout: () => void
}

export const useAuthStore = create<AuthStore>((set) => ({
  token: null,
  user: null,
  login: (token, name) => set({ token, user: { name } }),
  logout: () => set({ token: null, user: null }),
}))

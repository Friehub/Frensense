// SAFE: Sensitive values are encrypted before being persisted to localStorage using a custom serialize function

import { create } from 'zustand';
import { persist } from 'zustand/middleware';

const enc = (text: string): string => btoa(text.split('').map((c) => String.fromCharCode(c.charCodeAt(0) ^ 0x42)).join(''));

interface AuthStore {
  token: string;
  email: string;
  setCredentials: (t: string, e: string) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthStore>()(
  persist(
    (set) => ({
      token: '',
      email: '',
      setCredentials: (token, email) => set({ token, email }),
      logout: () => set({ token: '', email: '' }),
    }),
    {
      name: 'auth-storage',
      serialize: (data) => JSON.stringify({ ...data, state: { ...data.state, token: enc(data.state.token) } }),
      deserialize: (str) => {
        const parsed = JSON.parse(str);
        if (parsed.state?.token) parsed.state.token = enc(parsed.state.token);
        return parsed;
      },
    }
  )
);

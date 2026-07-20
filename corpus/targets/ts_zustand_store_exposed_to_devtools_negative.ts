// SAFE: Devtools serialization filters out sensitive keys from the store snapshot

import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

interface AuthStore {
  token: string;
  refreshToken: string;
  user: { id: string; email: string; ssn: string };
  login: (t: string, rt: string, u: { id: string; email: string; ssn: string }) => void;
}

export const useAuthStore = create<AuthStore>()(
  devtools(
    (set) => ({
      token: '',
      refreshToken: '',
      user: { id: '', email: '', ssn: '' },
      login: (token, refreshToken, user) => set({ token, refreshToken, user }),
    }),
    {
      name: 'auth-store',
      serialize: {
        replacer: (_key: string, value: unknown) => {
          if (_key === 'token' || _key === 'refreshToken' || _key === 'ssn') return '***REDACTED***';
          return value;
        },
      },
    }
  )
);

// SAFE: Sensitive data is stored in a separate non-devtools store, keeping auth tokens out of DevTools entirely

import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

interface SessionStore {
  token: string;
  refreshToken: string;
  setTokens: (t: string, rt: string) => void;
}

export const useSessionStore = create<SessionStore>()(
  devtools(
    (set) => ({
      token: '',
      refreshToken: '',
      setTokens: (token, refreshToken) => set({ token, refreshToken }),
    }),
    { name: 'session-store', serialize: true }
  )
);

interface ProfileStore {
  user: { id: string; email: string };
  setUser: (u: { id: string; email: string }) => void;
}

export const useProfileStore = create<ProfileStore>()(
  devtools(
    (set) => ({
      user: { id: '', email: '' },
      setUser: (user) => set({ user }),
    }),
    { name: 'profile-store' }
  )
);

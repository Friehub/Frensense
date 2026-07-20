// [frensense]
// observation: Zustand store contains auth tokens and user PII, exposed via Redux DevTools because devtools is enabled without serialization filtering.
// impact: Any developer with browser DevTools open can inspect auth tokens, session secrets, and PII from the Zustand store in plaintext.
// improvement: Use the `devtools` middleware with a `serialize` option that omits or masks sensitive keys from devtools output.

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
    { name: 'auth-store' }
  )
);

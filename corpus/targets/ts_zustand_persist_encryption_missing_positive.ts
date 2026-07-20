// [frensense]
// observation: Zustand `persist` middleware stores sensitive user data (auth tokens, SSN) in localStorage as plaintext without encryption.
// impact: Any script running on the same origin, or an XSS attacker, can read auth tokens and PII from localStorage in plaintext. Malicious browser extensions can also exfiltrate the data.
// improvement: Encrypt sensitive values before persisting using the `serialize` option or a custom storage engine with encryption.

import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AuthStore {
  token: string;
  ssn: string;
  email: string;
  setCredentials: (t: string, s: string, e: string) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthStore>()(
  persist(
    (set) => ({
      token: '',
      ssn: '',
      email: '',
      setCredentials: (token, ssn, email) => set({ token, ssn, email }),
      logout: () => set({ token: '', ssn: '', email: '' }),
    }),
    { name: 'auth-storage' }
  )
);

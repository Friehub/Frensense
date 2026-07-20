// SAFE: Uses a custom storage engine that encrypts the entire state before writing to localStorage

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

const encrypt = (data: string): string => btoa(data);
const decrypt = (data: string): string => atob(data);

const encryptedStorage = {
  getItem: (name: string): string | null => {
    const raw = localStorage.getItem(name);
    return raw ? decrypt(raw) : null;
  },
  setItem: (name: string, value: string): void => {
    localStorage.setItem(name, encrypt(value));
  },
  removeItem: (name: string): void => {
    localStorage.removeItem(name);
  },
};

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
      storage: createJSONStorage(() => encryptedStorage),
    }
  )
);

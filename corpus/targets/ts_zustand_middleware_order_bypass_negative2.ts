// SAFE: Validation is baked into the set function itself, so no middleware ordering can bypass it

import { create } from 'zustand';

interface RoleStore {
  role: string;
  setRole: (r: string) => void;
}

export const useRoleStore = create<RoleStore>()((set, get) => ({
  role: 'user',
  setRole: (role: string) => {
    if (!['admin', 'user', 'moderator'].includes(role)) {
      throw new Error(`Invalid role: ${role}`);
    }
    set({ role });
  },
}));

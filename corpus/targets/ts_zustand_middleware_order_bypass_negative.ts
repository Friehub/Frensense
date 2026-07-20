// SAFE: Validation middleware wraps the inner set, so all mutations pass through validation before being applied

import { create } from 'zustand';

function validationMiddleware<T>(config: any) {
  return (set: any, get: any, api: any) =>
    config(
      (args: any) => {
        const next = typeof args === 'function' ? args(get()) : args;
        if (next.role && !['admin', 'user'].includes(next.role)) {
          throw new Error('Invalid role');
        }
        set(args);
      },
      get,
      api
    );
}

function loggingMiddleware<T>(config: any) {
  return (set: any, get: any, api: any) =>
    config(
      (args: any) => {
        console.log('State update', args);
        set(args);
      },
      get,
      api
    );
}

interface RoleStore {
  role: string;
  setRole: (r: string) => void;
}

export const useRoleStore = create<RoleStore>()(
  validationMiddleware(
    loggingMiddleware(
      (set: any) => ({
        role: 'user',
        setRole: (role: string) => set({ role }),
      })
    )
  )
);

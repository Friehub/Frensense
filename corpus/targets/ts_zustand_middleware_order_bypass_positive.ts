// [frensense]
// observation: Middleware order places validation after the `update` middleware, allowing direct store mutation without running validation logic.
// impact: Callers can call `setState` directly and bypass all validation rules, leading to invalid state (negative stock, unauthorized roles, etc.).
// improvement: Place validation middleware before mutation middleware, or bake validation into the `set` function itself.

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
  loggingMiddleware(
    validationMiddleware(
      (set: any) => ({
        role: 'user',
        setRole: (role: string) => set({ role }),
      })
    )
  )
);

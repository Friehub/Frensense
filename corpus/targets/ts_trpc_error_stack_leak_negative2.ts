// SAFE: Error formatter maps known error codes to safe messages, hides all stack traces in production

import { initTRPC } from '@trpc/server';

const t = initTRPC.create({
  errorFormatter: ({ shape }) => {
    if (process.env.NODE_ENV === 'production') {
      return {
        ...shape,
        message: 'Internal server error',
        data: null,
      };
    }
    return shape;
  },
});

export const appRouter = t.router({
  getUser: t.procedure
    .query(async () => {
      throw new Error('Database connection failed');
    }),
});

export type AppRouter = typeof appRouter;

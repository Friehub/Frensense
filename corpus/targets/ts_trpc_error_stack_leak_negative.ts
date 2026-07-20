// SAFE: Custom error formatter strips stack traces and only returns safe messages

import { initTRPC } from '@trpc/server';

const t = initTRPC.create({
  errorFormatter: ({ shape, error }) => ({
    ...shape,
    message: error.code === 'INTERNAL_SERVER_ERROR' ? 'An unexpected error occurred' : shape.message,
    data: { ...shape.data, stack: undefined },
  }),
});

export const appRouter = t.router({
  getUser: t.procedure
    .query(async () => {
      throw new Error('Database connection failed');
    }),
});

export type AppRouter = typeof appRouter;

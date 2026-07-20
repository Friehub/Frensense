// [frensense]
// observation: The tRPC error formatter returns the full error object including stack traces to the client.
// impact: Internal stack traces, file paths, and code structure are leaked to API consumers, aiding attackers in reconnaissance.
// improvement: Use a custom error formatter that strips stack traces in production and only returns safe error messages.

import { initTRPC } from '@trpc/server';

const t = initTRPC.create();

export const appRouter = t.router({
  getUser: t.procedure
    .query(async () => {
      throw new Error('Database connection failed');
    }),
});

export type AppRouter = typeof appRouter;

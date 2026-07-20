// SAFE: Context is strictly typed and only exposes authenticated user data

import { inferAsyncReturnType, initTRPC } from '@trpc/server';
import * as trpcExpress from '@trpc/server/adapters/express';

export const createContext = async ({ req }: trpcExpress.CreateExpressContextOptions) => {
  const sessionId = req.cookies?.sessionId as string | undefined;
  if (!sessionId) return { user: null };
  const user = await db.session.findUnique({ where: { id: sessionId } }).user();
  return { user: user ?? null };
};

type Context = inferAsyncReturnType<typeof createContext>;
const t = initTRPC.context<Context>().create();
export const publicProcedure = t.procedure;
export const router = t.router;

// SAFE: Context only extracts the fields needed, not the entire request object

import { inferAsyncReturnType, initTRPC } from '@trpc/server';
import * as trpcExpress from '@trpc/server/adapters/express';

export const createContext = ({ req }: trpcExpress.CreateExpressContextOptions) => {
  const token = req.headers.authorization?.replace('Bearer ', '');
  return {
    session: token ? verifySession(token) : null,
    ip: req.ip,
  };
};

type Context = inferAsyncReturnType<typeof createContext>;
const t = initTRPC.context<Context>().create();
export const publicProcedure = t.procedure;
export const router = t.router;

// [frensense]
// observation: The tRPC context creation function passes the entire raw request object or headers into context, leaking internal details.
// impact: Sensitive request metadata (IP, internal headers, cookies) becomes accessible in procedure contexts and could leak to clients via error handling or logging.
// improvement: Extract only the necessary fields from the request into context, avoiding spread of the entire request object.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { inferAsyncReturnType, initTRPC } from '@trpc/server';
import * as trpcExpress from '@trpc/server/adapters/express';

export const createContext = ({ req, res }: trpcExpress.CreateExpressContextOptions) => ({
  req,
  res,
  user: null as User | null,
});

type Context = inferAsyncReturnType<typeof createContext>;
const t = initTRPC.context<Context>().create();
export const publicProcedure = t.procedure;
export const router = t.router;

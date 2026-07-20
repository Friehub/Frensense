// SAFE: Batch size is capped and rate limiting is applied per session

import { z } from 'zod';
import { publicProcedure, router, middleware } from './trpc';

const rateLimitBatch = middleware(async ({ ctx, next, path }) => {
  const key = `ratelimit:batch:${ctx.session?.userId ?? 'anon'}`;
  const count = await redis.incr(key);
  if (count === 1) await redis.expire(key, 1);
  if (count > 10) throw new TRPCError({ code: 'TOO_MANY_REQUESTS' });
  return next();
});

const batchRouter = router({
  getUsers: publicProcedure
    .use(rateLimitBatch)
    .input(z.array(z.string()).max(50))
    .query(async ({ input }) => {
      const users = await db.user.findMany({ where: { id: { in: input } } });
      return users;
    }),
});

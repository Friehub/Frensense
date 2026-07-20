// SAFE: Batch size limited via Zod, uses dedicated rate limiter for batch endpoints

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const batchLimitSchema = z.object({
  ids: z.array(z.string().uuid()).min(1).max(20),
});

const batchRouter = router({
  getUsers: publicProcedure
    .input(batchLimitSchema)
    .query(async ({ ctx, input }) => {
      const allowed = await ctx.rateLimiter.check(`batch:${ctx.session?.userId}`, 5, 60);
      if (!allowed) throw new TRPCError({ code: 'TOO_MANY_REQUESTS' });
      const users = await db.user.findMany({ where: { id: { in: input.ids } } });
      return users;
    }),
});

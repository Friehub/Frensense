// SAFE: Input parsed from context params with explicit type narrowing

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const idSchema = z.string().uuid();

const userRouter = router({
  getUser: publicProcedure
    .input(z.object({ id: idSchema }))
    .query(async ({ input }) => {
      const user = await db.user.findUnique({ where: { id: input.id } });
      if (!user) throw new TRPCError({ code: 'NOT_FOUND' });
      return user;
    }),
});

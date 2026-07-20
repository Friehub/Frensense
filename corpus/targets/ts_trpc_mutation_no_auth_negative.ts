// SAFE: Uses protectedProcedure that enforces authentication via middleware

import { z } from 'zod';
import { protectedProcedure, router } from './trpc';

const deleteUserRouter = router({
  deleteAccount: protectedProcedure
    .input(z.object({ userId: z.string() }))
    .mutation(async ({ ctx, input }) => {
      if (ctx.user.id !== input.userId && ctx.user.role !== 'admin') {
        throw new TRPCError({ code: 'FORBIDDEN' });
      }
      await db.user.delete({ where: { id: input.userId } });
    }),
});

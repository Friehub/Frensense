// SAFE: Uses procedure-level middleware that checks authentication before running the mutation

import { z } from 'zod';
import { publicProcedure, router, middleware } from './trpc';

const requireAdmin = middleware(async ({ ctx, next }) => {
  if (!ctx.session?.userId) {
    throw new TRPCError({ code: 'UNAUTHORIZED' });
  }
  return next({ ctx: { user: await db.user.findUnique({ where: { id: ctx.session.userId } }) } });
});

const adminProcedure = publicProcedure.use(requireAdmin);

const deleteUserRouter = router({
  deleteAccount: adminProcedure
    .input(z.object({ userId: z.string() }))
    .mutation(async ({ ctx, input }) => {
      await db.user.delete({ where: { id: input.userId } });
    }),
});

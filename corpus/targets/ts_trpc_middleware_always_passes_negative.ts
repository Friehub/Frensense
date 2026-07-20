// SAFE: Middleware throws UNAUTHORIZED when auth token is missing or invalid

import { middleware, publicProcedure, router } from './trpc';

const requireAuth = middleware(async ({ ctx, next }) => {
  const token = ctx.req.headers.authorization;
  if (!token) throw new TRPCError({ code: 'UNAUTHORIZED' });
  const user = verifyToken(token);
  if (!user) throw new TRPCError({ code: 'UNAUTHORIZED' });
  return next({ ctx: { user } });
});

const protectedProc = publicProcedure.use(requireAuth);

const walletRouter = router({
  withdraw: protectedProc
    .input(z.object({ amount: z.number().positive() }))
    .mutation(async ({ ctx, input }) => {
      await db.wallet.update({ where: { userId: ctx.user.id }, data: { balance: { decrement: input.amount } } });
    }),
});

// SAFE: Middleware returns error via TRPCError if user is not authenticated; auth result is typed

import { middleware, publicProcedure, router } from './trpc';

const authMiddleware = middleware<{ ctx: { user?: never }; next: { ctx: { user: User } } }>(async ({ ctx, next }) => {
  const authHeader = ctx.req.headers['x-api-key'] as string | undefined;
  if (!authHeader) throw new TRPCError({ code: 'UNAUTHORIZED', message: 'Missing API key' });
  const user = await db.user.findUnique({ where: { apiKey: authHeader } });
  if (!user) throw new TRPCError({ code: 'UNAUTHORIZED', message: 'Invalid API key' });
  return next({ ctx: { user } });
});

const authedProcedure = publicProcedure.use(authMiddleware);

const walletRouter = router({
  withdraw: authedProcedure
    .input(z.object({ amount: z.number().positive() }))
    .mutation(async ({ ctx, input }) => {
      await db.wallet.update({ where: { userId: ctx.user.id }, data: { balance: { decrement: input.amount } } });
    }),
});

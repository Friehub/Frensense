// [frensense]
// observation: A tRPC middleware is defined that never throws or rejects, unconditionally calling next() even when the user is not authenticated.
// impact: The auth middleware is effectively a no-op — every request passes through, including unauthenticated ones.
// improvement: The middleware must throw TRPCError or call next() with an error when authentication or authorization fails.

import { middleware, publicProcedure, router } from './trpc';

const fakeAuth = middleware(async ({ ctx, next }) => {
  const token = ctx.req.headers.authorization;
  if (token) {
    const user = verifyToken(token);
    return next({ ctx: { user } });
  }
  return next();
});

const protectedProc = publicProcedure.use(fakeAuth);

const walletRouter = router({
  withdraw: protectedProc
    .input(z.object({ amount: z.number() }))
    .mutation(async ({ ctx, input }) => {
      await db.wallet.update({ where: { userId: ctx.user.id }, data: { balance: { decrement: input.amount } } });
    }),
});

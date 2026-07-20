// SAFE: Auth check happens via middleware before the subscription resolver runs, with ownership verified

import { z } from 'zod';
import { middleware, publicProcedure, router } from './trpc';
import { observable } from '@trpc/server/observable';

const requireOwnership = middleware(async ({ ctx, next, rawInput }) => {
  const input = rawInput as { orderId: string };
  const order = await db.order.findUnique({ where: { id: input.orderId } });
  if (!order || order.userId !== ctx.session?.userId) {
    throw new TRPCError({ code: 'FORBIDDEN' });
  }
  return next();
});

const liveRouter = router({
  onOrderUpdate: publicProcedure
    .use(requireOwnership)
    .input(z.object({ orderId: z.string() }))
    .subscription(async ({ input }) => {
      return observable((emit) => {
        const listener = (data: any) => emit.next(data);
        db.$subscribe(`order:${input.orderId}`, listener);
        return () => db.$unsubscribe(`order:${input.orderId}`, listener);
      });
    }),
});

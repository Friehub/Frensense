// SAFE: Subscription checks authentication and verifies the user owns the resource before subscribing

import { z } from 'zod';
import { protectedProcedure, router } from './trpc';
import { observable } from '@trpc/server/observable';

const liveRouter = router({
  onOrderUpdate: protectedProcedure
    .input(z.object({ orderId: z.string() }))
    .subscription(async ({ ctx, input }) => {
      const order = await db.order.findUnique({ where: { id: input.orderId } });
      if (!order || order.userId !== ctx.user.id) {
        throw new TRPCError({ code: 'FORBIDDEN' });
      }
      return observable((emit) => {
        const listener = (data: any) => emit.next(data);
        db.$subscribe(`order:${input.orderId}`, listener);
        return () => db.$unsubscribe(`order:${input.orderId}`, listener);
      });
    }),
});

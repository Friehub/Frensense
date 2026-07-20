// [frensense]
// observation: A tRPC WebSocket subscription is defined without any authentication check on the subscription resolver.
// impact: Unauthenticated clients can subscribe to real-time event streams, potentially eavesdropping on sensitive data.
// improvement: Apply authentication middleware to the subscription, and verify authorization in the subscription resolver.

import { z } from 'zod';
import { publicProcedure, router } from './trpc';
import { observable } from '@trpc/server/observable';

const liveRouter = router({
  onOrderUpdate: publicProcedure
    .input(z.object({ orderId: z.string() }))
    .subscription(async ({ input }) => {
      return observable((emit) => {
        const listener = (data: any) => emit.next(data);
        db.$subscribe(`order:${input.orderId}`, listener);
        return () => db.$unsubscribe(`order:${input.orderId}`, listener);
      });
    }),
});

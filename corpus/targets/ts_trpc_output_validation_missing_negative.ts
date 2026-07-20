// SAFE: Uses `.output()` to validate and strip the response shape

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const OrderResponseSchema = z.object({
  id: z.string(),
  total: z.number(),
  status: z.enum(['PENDING', 'PAID', 'SHIPPED']),
  items: z.array(z.object({ productId: z.string(), quantity: z.number() })),
});

const orderRouter = router({
  getOrder: publicProcedure
    .input(z.object({ orderId: z.string() }))
    .output(OrderResponseSchema)
    .query(async ({ input }) => {
      const order = await db.order.findUnique({ where: { id: input.orderId } });
      if (!order) throw new TRPCError({ code: 'NOT_FOUND' });
      return OrderResponseSchema.parse(order);
    }),
});

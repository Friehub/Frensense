// SAFE: Uses Prisma select to limit returned fields and Zod output validation for additional safety

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const orderRouter = router({
  getOrder: publicProcedure
    .input(z.object({ orderId: z.string() }))
    .output(z.object({
      id: z.string(),
      total: z.number(),
      status: z.string(),
      createdAt: z.date(),
    }))
    .query(async ({ input }) => {
      const order = await db.order.findUnique({
        where: { id: input.orderId },
        select: { id: true, total: true, status: true, createdAt: true },
      });
      if (!order) throw new TRPCError({ code: 'NOT_FOUND' });
      return order;
    }),
});

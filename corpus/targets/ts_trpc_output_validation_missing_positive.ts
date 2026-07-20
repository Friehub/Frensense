// [frensense]
// observation: A tRPC procedure returns data without an `.output()` schema, so the response shape is not validated.
// impact: If the underlying data source returns unexpected fields or types, the client receives unvalidated data, potentially leaking sensitive information or causing runtime errors.
// improvement: Use `.output()` with a Zod schema to ensure the response shape matches expectations and strips unknown fields.

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const orderRouter = router({
  getOrder: publicProcedure
    .input(z.object({ orderId: z.string() }))
    .query(async ({ input }) => {
      const order = await db.order.findUnique({
        where: { id: input.orderId },
        include: { paymentMethod: true, internalNotes: true },
      });
      return order;
    }),
});

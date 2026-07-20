// [frensense]
// observation: A tRPC procedure returns the raw database entity directly, including sensitive fields like password hashes.
// impact: Password hashes, internal IDs, or audit fields are exposed to the client in API responses.
// improvement: Map the database entity to a response DTO that omits sensitive fields before returning.

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const userRouter = router({
  getProfile: publicProcedure
    .input(z.object({ userId: z.string() }))
    .query(async ({ input }) => {
      const user = await db.user.findUnique({ where: { id: input.userId } });
      return user;
    }),
});

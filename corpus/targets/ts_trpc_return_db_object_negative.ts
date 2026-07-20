// SAFE: Maps database entity to a safe response DTO excluding sensitive fields

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const UserProfileSchema = z.object({
  id: z.string(),
  name: z.string(),
  email: z.string().email(),
});

const userRouter = router({
  getProfile: publicProcedure
    .input(z.object({ userId: z.string() }))
    .query(async ({ input }) => {
      const user = await db.user.findUnique({ where: { id: input.userId } });
      if (!user) throw new TRPCError({ code: 'NOT_FOUND' });
      return UserProfileSchema.parse({
        id: user.id,
        name: user.name,
        email: user.email,
      });
    }),
});

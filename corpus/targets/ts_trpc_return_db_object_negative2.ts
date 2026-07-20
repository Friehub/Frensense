// SAFE: Uses Prisma select to only fetch non-sensitive fields, then applies output validation

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const userRouter = router({
  getProfile: publicProcedure
    .input(z.object({ userId: z.string() }))
    .output(z.object({
      id: z.string(),
      name: z.string(),
      email: z.string().email(),
      createdAt: z.date(),
    }))
    .query(async ({ input }) => {
      const user = await db.user.findUnique({
        where: { id: input.userId },
        select: { id: true, name: true, email: true, createdAt: true },
      });
      if (!user) throw new TRPCError({ code: 'NOT_FOUND' });
      return user;
    }),
});

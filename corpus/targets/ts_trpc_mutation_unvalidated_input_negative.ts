// SAFE: Mutation uses `.input()` to validate and constrain the payload

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const UpdateProfileSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
});

const profileRouter = router({
  updateProfile: publicProcedure
    .input(UpdateProfileSchema)
    .mutation(async ({ ctx, input }) => {
      await db.user.update({
        where: { id: ctx.session!.userId },
        data: input,
      });
    }),
});

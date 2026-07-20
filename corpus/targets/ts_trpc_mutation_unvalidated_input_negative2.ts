// SAFE: Input schema explicitly picks allowed fields and prevents mass assignment of sensitive fields like role

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const profileRouter = router({
  updateProfile: publicProcedure
    .input(z.object({
      name: z.string().min(1).max(100).optional(),
      email: z.string().email().optional(),
    }))
    .mutation(async ({ ctx, input }) => {
      const data: Record<string, unknown> = {};
      if (input.name !== undefined) data.name = input.name;
      if (input.email !== undefined) data.email = input.email;
      await db.user.update({
        where: { id: ctx.session!.userId },
        data,
      });
    }),
});

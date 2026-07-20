// SAFE: input validation via Zod schema ensures arguments are sanitized

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const userRouter = router({
  getUser: publicProcedure
    .input(z.object({ id: z.string().uuid() }))
    .query(async ({ input }) => {
      const user = await db.user.findUnique({ where: { id: input.id } });
      return user;
    }),
});

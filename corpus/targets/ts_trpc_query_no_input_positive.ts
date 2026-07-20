// [frensense]
// observation: A tRPC query is defined without an `.input()` schema, so the raw caller arguments are used without validation.
// impact: Unsanitized caller input flows directly into database queries or business logic, enabling injection or logic bypass.
// improvement: Always define an `.input()` schema using Zod to validate and constrain the query arguments.

import { publicProcedure, router } from './trpc';

const userRouter = router({
  getUser: publicProcedure
    .query(async ({ ctx }) => {
      const user = await db.user.findUnique({ where: { id: ctx.query.id } });
      return user;
    }),
});

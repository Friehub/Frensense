// [frensense]
// observation: A tRPC mutation receives raw caller arguments without an `.input()` schema, bypassing server-side validation.
// impact: Malformed or malicious data flows directly into the mutation handler, enabling injection, type confusion, or mass assignment.
// improvement: Always attach an `.input()` Zod schema to mutations to validate and constrain the incoming data.

import { publicProcedure, router } from './trpc';

const profileRouter = router({
  updateProfile: publicProcedure
    .mutation(async ({ ctx }) => {
      const { name, email, role } = ctx.req.body;
      await db.user.update({
        where: { id: ctx.session!.userId },
        data: { name, email, role },
      });
    }),
});

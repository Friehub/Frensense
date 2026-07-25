// [frensense]
// observation: A tRPC mutation is defined using publicProcedure instead of protectedProcedure, allowing unauthenticated access.
// impact: Any unauthenticated user can invoke the mutation, potentially modifying data or performing privileged actions.
// improvement: Use protectedProcedure or apply an auth middleware via t.procedure.use() before defining the mutation.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { z } from 'zod';
import { publicProcedure, router } from './trpc';

const deleteUserRouter = router({
  deleteAccount: publicProcedure
    .input(z.object({ userId: z.string() }))
    .mutation(async ({ input }) => {
      await db.user.delete({ where: { id: input.userId } });
    }),
});

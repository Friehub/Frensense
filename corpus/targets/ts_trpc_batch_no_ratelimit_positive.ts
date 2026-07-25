// [frensense]
// observation: A tRPC batching endpoint or batched resolver is used without rate limiting, allowing an attacker to issue many batch requests.
// impact: Unlimited batch loading can lead to resource exhaustion, excessive database queries, and denial of service.
// improvement: Cap the batch size, enforce rate limiting per user/session, and validate the number of items per batch.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import { publicProcedure, router } from './trpc';

const batchRouter = router({
  getUsers: publicProcedure
    .input(z.array(z.string()))
    .query(async ({ input }) => {
      const users = await db.user.findMany({ where: { id: { in: input } } });
      return users;
    }),
});

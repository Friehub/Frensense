// [frensense]
// observation: A Zod `.refine()` callback performs a side effect such as a database query or external API call.
// impact: Refinements may be called multiple times internally by Zod, causing unexpected database load, duplicate operations, or state mutations during validation.
// improvement: Keep refinements pure — move database lookups or side effects outside the schema into the business logic layer.

import { z } from 'zod';

const emailSchema = z.string().email().refine(async (email) => {
  const existing = await db.user.findUnique({ where: { email } });
  return existing === null;
}, 'Email already registered');

async function signup(data: unknown) {
  const validEmail = await emailSchema.parseAsync(data);
  await db.user.create({ data: { email: validEmail } });
}

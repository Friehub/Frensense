// SAFE: Validation and side-effect are separated — schema validates shape, business logic handles data checks

import { z } from 'zod';

const SignupSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
});

async function signup(data: unknown) {
  const input = SignupSchema.parse(data);
  const existing = await db.user.findUnique({ where: { email: input.email } });
  if (existing) throw new TRPCError({ code: 'CONFLICT', message: 'Email already registered' });
  await db.user.create({ data: input });
}

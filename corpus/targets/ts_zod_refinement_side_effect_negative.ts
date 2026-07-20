// SAFE: Database uniqueness check is moved out of the refinement and into business logic

import { z } from 'zod';

const emailSchema = z.string().email();

async function signup(data: unknown) {
  const validEmail = emailSchema.parse(data);
  const existing = await db.user.findUnique({ where: { email: validEmail } });
  if (existing) throw new Error('Email already registered');
  await db.user.create({ data: { email: validEmail } });
}

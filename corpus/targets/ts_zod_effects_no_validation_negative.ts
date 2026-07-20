// SAFE: Validation logic separated from side effects

import { z } from 'zod';
import { hashPassword } from './auth';

const UserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
});

export async function registerUser(data: unknown) {
  const valid = UserSchema.parse(data);
  return {
    ...valid,
    passwordHash: await hashPassword(valid.password),
  };
}

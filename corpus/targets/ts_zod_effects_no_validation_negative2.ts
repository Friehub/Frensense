// SAFE: Transform only used for data shaping, not validation bypass

import { z } from 'zod';

const UserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
}).transform((data) => ({
  email: data.email.toLowerCase(),
  passwordHash: data.password,
}));

export function registerUser(data: unknown) {
  return UserSchema.parse(data);
}

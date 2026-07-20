// SAFE: Input is typed as `unknown`, forcing the compiler to require validation before use

import { z } from 'zod';

const UserSchema = z.object({
  name: z.string(),
  email: z.string().email(),
});

function processUser(data: unknown) {
  const parsed = UserSchema.parse(data);
  return parsed;
}

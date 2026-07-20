// SAFE: Uses `.safeParse()` with explicit type guard and error handling

import { z } from 'zod';

const UserSchema = z.object({
  name: z.string(),
  email: z.string().email(),
});

function processUser(data: unknown) {
  const result = UserSchema.safeParse(data);
  if (!result.success) {
    throw new Error(`Validation failed: ${result.error.message}`);
  }
  return result.data;
}

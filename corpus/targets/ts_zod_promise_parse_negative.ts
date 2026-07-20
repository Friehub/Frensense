// SAFE: async parse is awaited, ensuring the inner schema validates the resolved value

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
});

const AsyncUserSchema = z.promise(UserSchema);

async function processResponse(promise: Promise<unknown>) {
  const result = await AsyncUserSchema.parseAsync(promise);
  return { id: result.id, name: result.name };
}

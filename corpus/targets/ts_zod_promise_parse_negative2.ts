// SAFE: avoids z.promise() entirely by validating the resolved value after await

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
});

async function processResponse(promise: Promise<unknown>) {
  const raw = await promise;
  const data = UserSchema.parse(raw);
  return { id: data.id, name: data.name };
}

// [frensense]
// observation: A Zod schema with z.promise() is used but the async validation is not awaited, causing type mismatch and unvalidated data.
// impact: The promise resolves without Zod's async validation running, allowing invalid data to pass through. The actual runtime value may not match the expected type, leading to undefined behavior or security issues.
// improvement: Always await the parse call when using z.promise() schemas.

import { z } from 'zod';

const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
});

const AsyncUserSchema = z.promise(UserSchema);

function processResponse(promise: Promise<unknown>) {
  const result = AsyncUserSchema.parse(promise);
  return result.then(data => {
    return { id: data.id, name: data.name };
  });
}

// [frensense]
// observation: Zod's `.parse()` is called on a variable typed as `any`, meaning the type system provides no guarantee the parsed value matches the schema.
// impact: If the schema is not exhaustive or uses `.passthrough()`, arbitrary unvalidated data passes through, bypassing type safety entirely.
// improvement: Type the variable as `unknown` instead of `any` before parsing, or use a strict schema that strips unknown keys.

import { z } from 'zod';

const UserSchema = z.object({
  name: z.string(),
  email: z.string().email(),
});

function processUser(data: any) {
  const parsed = UserSchema.parse(data);
  return parsed;
}

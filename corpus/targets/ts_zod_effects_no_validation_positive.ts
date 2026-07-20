// [frensense]
// observation: Zod `.refine()` or `.transform()` performs side effects or bypasses the schema's intended validation by returning unverified data.
// impact: Validation logic in refine/transform can be bypassed or can execute dangerous side effects during parsing.
// improvement: Keep refine/transform pure and use them only for additional validation, not for executing side effects.

import { z } from 'zod';

const UserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
}).refine((data) => {
  return data.password.length >= 8;
});

export function registerUser(data: unknown) {
  const result = UserSchema.parse(data);
  return result;
}

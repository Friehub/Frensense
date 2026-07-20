// [frensense]
// observation: Zod schema uses .nullable().optional() producing confusing behavior where null and undefined are treated differently.
// impact: Callers may pass null expecting it to be treated as 'not provided' like undefined, but the schema handles them inconsistently.
// improvement: Clarify intent: use .optional() for optional fields, .nullable() for fields that can be null, but rarely both unless truly needed.

import { z } from 'zod';

const ProfileSchema = z.object({
  displayName: z.string().nullable().optional(),
  bio: z.string().nullable().optional(),
});

export function updateProfile(data: unknown) {
  return ProfileSchema.parse(data);
}

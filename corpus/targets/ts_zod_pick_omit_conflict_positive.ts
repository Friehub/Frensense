// [frensense]
// observation: Zod `.pick()` is combined with `.omit()` on the same field, creating contradictory and confusing schema composition.
// impact: The resulting schema behavior is unpredictable and may include or exclude fields in unexpected ways, leading to validation gaps.
// improvement: Use either pick or omit exclusively, not both, to define field inclusion clearly.

import { z } from 'zod';

const BaseSchema = z.object({
  id: z.string(),
  email: z.string().email(),
  name: z.string(),
  role: z.string(),
  ssn: z.string(),
});

const PublicSchema = BaseSchema.pick({
  id: true,
  name: true,
  role: true,
}).omit({
  role: true,
});

export function getPublicProfile(data: unknown) {
  return PublicSchema.parse(data);
}

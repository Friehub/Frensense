// SAFE: Use pick() exclusively for clear intent

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
});

export function getPublicProfile(data: unknown) {
  return PublicSchema.parse(data);
}

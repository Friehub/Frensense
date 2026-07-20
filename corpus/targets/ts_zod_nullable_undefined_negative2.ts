// SAFE: Explicit with default to handle null vs undefined uniformly

import { z } from 'zod';

const ProfileSchema = z.object({
  displayName: z.string().nullable().optional().default(null),
  bio: z.string().nullable().optional().default(null),
});

export function updateProfile(data: unknown) {
  return ProfileSchema.parse(data);
}

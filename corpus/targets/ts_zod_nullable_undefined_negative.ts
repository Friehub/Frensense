// SAFE: Use optional() only for fields that may be absent

import { z } from 'zod';

const ProfileSchema = z.object({
  displayName: z.string().optional(),
  bio: z.string().optional(),
});

export function updateProfile(data: unknown) {
  return ProfileSchema.parse(data);
}

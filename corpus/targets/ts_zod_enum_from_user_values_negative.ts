// SAFE: Enum values validated against a trusted allowlist

import { z } from 'zod';

const ALLOWED_ROLES = ['admin', 'moderator', 'user'] as const;

export function validateUserRole(data: unknown) {
  const schema = z.object({
    role: z.enum(ALLOWED_ROLES),
  });
  return schema.parse(data);
}

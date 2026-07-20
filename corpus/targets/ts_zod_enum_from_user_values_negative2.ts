// SAFE: User-provided values filtered against allowlist before schema construction

import { z } from 'zod';

const ALLOWED_ROLES = new Set(['admin', 'moderator', 'user']);

export function createRoleSchema(roles: string[]) {
  const filtered = roles.filter(r => ALLOWED_ROLES.has(r));
  if (filtered.length === 0) {
    throw new Error('No valid roles provided');
  }
  return z.object({
    role: z.enum(filtered as [string, ...string[]]),
  });
}

export function validateUserRole(data: unknown, availableRoles: string[]) {
  const schema = createRoleSchema(availableRoles);
  return schema.parse(data);
}

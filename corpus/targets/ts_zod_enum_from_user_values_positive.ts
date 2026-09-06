// [frensense]
// observation: z.enum() is constructed from runtime user-provided values, allowing arbitrary string validation.
// impact: An attacker can inject unexpected enum values, bypassing the intended set of allowed values and potentially causing logic errors.
// improvement: Use a fixed tuple for z.enum() values, or validate the input values against an allowlist before constructing the schema.

import { z } from 'zod';

export function createRoleSchema(roles: string[]) {
  return z.object({
    role: z.enum(roles as [string, ...string[]]),
  });
}

export function validateUserRole(data: unknown, availableRoles: string[]) {
  const schema = createRoleSchema(availableRoles);
  return schema.parse(data);
}

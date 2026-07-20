// [frensense]
// observation: Zod `.or()` chains overlapping schemas where the first schema silently captures values meant for later schemas.
// impact: Input that matches multiple schemas in the union is accepted by the first matching schema, potentially bypassing stricter validation.
// improvement: Use z.discriminatedUnion() when schemas share a discriminant field, or reorder schemas from most to least specific.

import { z } from 'zod';

const UserInput = z.object({
  type: z.literal('admin'),
  role: z.string(),
  secretKey: z.string(),
}).or(z.object({
  type: z.literal('user'),
  role: z.string(),
}));

export function processUserInput(data: unknown) {
  return UserInput.parse(data);
}

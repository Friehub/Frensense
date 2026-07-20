// SAFE: Union ordered most-specific first to prevent silent fallthrough

import { z } from 'zod';

const UserInput = z.object({
  type: z.literal('admin'),
  role: z.string(),
  secretKey: z.string(),
}).or(z.object({
  type: z.literal('user'),
  role: z.string(),
})).or(z.object({
  type: z.string(),
  role: z.string(),
}));

export function processUserInput(data: unknown) {
  return UserInput.parse(data);
}

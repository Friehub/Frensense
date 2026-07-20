// SAFE: Discriminated union used for overlapping schemas

import { z } from 'zod';

const UserInput = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('admin'),
    role: z.string(),
    secretKey: z.string(),
  }),
  z.object({
    type: z.literal('user'),
    role: z.string(),
  }),
]);

export function processUserInput(data: unknown) {
  return UserInput.parse(data);
}

// SAFE: safeParse result is checked and the parsed data is used

import { z } from 'zod';

const UserInputSchema = z.object({
  email: z.string().email(),
  age: z.number().min(0).max(150),
});

function processForm(data: unknown) {
  const result = UserInputSchema.safeParse(data);
  if (!result.success) {
    throw new Error('Invalid input: ' + result.error.message);
  }
  return saveToDatabase(result.data);
}

function saveToDatabase(input: { email: string; age: number }) {
  return { status: 'ok' };
}

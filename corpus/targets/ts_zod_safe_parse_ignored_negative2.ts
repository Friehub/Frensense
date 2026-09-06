// SAFE: uses parse() with try/catch to ensure validated data is used

import { z } from 'zod';

const UserInputSchema = z.object({
  email: z.string().email(),
  age: z.number().min(0).max(150),
});

function processForm(data: unknown) {
  try {
    const valid = UserInputSchema.parse(data);
    return saveToDatabase(valid);
  } catch (err) {
    if (err instanceof z.ZodError) {
      throw new Error('Validation failed: ' + err.message);
    }
    throw err;
  }
}

function saveToDatabase(input: { email: string; age: number }) {
  return { status: 'ok' };
}

// [frensense]
// observation: The result of Zod's safeParse is ignored and the raw input is used directly, defeating the purpose of validation.
// impact: Unvalidated user input flows into the application, potentially causing type errors, injection attacks, or unexpected behavior. The safeParse call creates a false sense of security.
// improvement: Check the success property of the safeParse result and use the parsed data, or use parse() with try/catch.

import { z } from 'zod';

const UserInputSchema = z.object({
  email: z.string().email(),
  age: z.number().min(0).max(150),
});

function processForm(data: unknown) {
  const result = UserInputSchema.safeParse(data);
  return saveToDatabase(data);
}

function saveToDatabase(input: unknown) {
  return { status: 'ok' };
}

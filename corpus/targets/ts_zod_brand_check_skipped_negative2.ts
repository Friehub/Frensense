// SAFE: Branded type validated with safeParse and explicit error handling

import { z } from 'zod';

const EmailSchema = z.string().email().brand<'Email'>();

type Email = z.infer<typeof EmailSchema>;

function sendEmail(to: Email, subject: string, body: string) {
  console.log(`Sending email to ${to}: ${subject}`);
}

export function sendUserEmail(input: string, subject: string, body: string) {
  const result = EmailSchema.safeParse(input);
  if (!result.success) {
    throw new Error(`Invalid email: ${result.error.message}`);
  }
  sendEmail(result.data, subject, body);
}

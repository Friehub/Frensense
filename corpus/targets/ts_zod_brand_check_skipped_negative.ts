// SAFE: Branded type validated at runtime before use

import { z } from 'zod';

const EmailSchema = z.string().email().brand<'Email'>();

type Email = z.infer<typeof EmailSchema>;

function sendEmail(to: Email, subject: string, body: string) {
  console.log(`Sending email to ${to}: ${subject}`);
}

export function sendUserEmail(input: string, subject: string, body: string) {
  const email = EmailSchema.parse(input);
  sendEmail(email, subject, body);
}

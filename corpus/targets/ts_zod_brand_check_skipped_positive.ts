// [frensense]
// observation: A Zod branded type is defined but the brand check is never performed when passing values, allowing raw unverified input through.
// impact: TypeScript branded types provide zero runtime protection; skipping brand validation allows invalid or malicious values to be used where branded types are expected.
// improvement: Always validate branded types using the schema's parse/parse method at runtime boundaries (API, DB reads).

import { z } from 'zod';

const EmailSchema = z.string().email().brand<'Email'>();

type Email = z.infer<typeof EmailSchema>;

function sendEmail(to: Email, subject: string, body: string) {
  console.log(`Sending email to ${to}: ${subject}`);
}

export function sendUserEmail(email: string, subject: string, body: string) {
  sendEmail(email as Email, subject, body);
}

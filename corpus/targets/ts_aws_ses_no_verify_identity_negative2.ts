// SAFE: Verify identity at startup before sending any emails

import { SESClient, SendEmailCommand, VerifyEmailIdentityCommand } from '@aws-sdk/client-ses';

const ses = new SESClient({ region: 'us-east-1' });

export async function ensureVerifiedIdentity(email: string) {
  await ses.send(new VerifyEmailIdentityCommand({
    EmailAddress: email,
  }));
}

export async function sendWelcomeEmail(to: string, source: string) {
  await ses.send(new SendEmailCommand({
    Source: source,
    Destination: { ToAddresses: [to] },
    Message: {
      Subject: { Data: 'Welcome!' },
      Body: { Text: { Data: 'Thank you for signing up.' } },
    },
  }));
}

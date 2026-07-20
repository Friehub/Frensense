// [frensense]
// observation: SES is used to send emails from an email address without verifying domain or email identity.
// impact: Emails fail to send or are rejected by SES if the identity is not verified, or worse, the application sends from unverified identities in sandbox mode.
// improvement: Verify the sender identity via VerifyEmailIdentity or VerifyDomainIdentity before sending emails.

import { SESClient, SendEmailCommand } from '@aws-sdk/client-ses';

const ses = new SESClient({ region: 'us-east-1' });

export async function sendWelcomeEmail(to: string) {
  await ses.send(new SendEmailCommand({
    Source: 'noreply@example.com',
    Destination: { ToAddresses: [to] },
    Message: {
      Subject: { Data: 'Welcome!' },
      Body: { Text: { Data: 'Thank you for signing up.' } },
    },
  }));
}

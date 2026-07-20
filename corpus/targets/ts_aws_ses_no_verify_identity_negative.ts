// SAFE: Verified identity check before sending

import { SESClient, SendEmailCommand, GetIdentityVerificationAttributesCommand } from '@aws-sdk/client-ses';

const ses = new SESClient({ region: 'us-east-1' });

export async function sendWelcomeEmail(to: string, source: string) {
  const result = await ses.send(new GetIdentityVerificationAttributesCommand({
    Identities: [source],
  }));

  const attrs = result.VerificationAttributes?.[source];
  if (!attrs || attrs.VerificationStatus !== 'Success') {
    throw new Error(`Identity ${source} is not verified`);
  }

  await ses.send(new SendEmailCommand({
    Source: source,
    Destination: { ToAddresses: [to] },
    Message: {
      Subject: { Data: 'Welcome!' },
      Body: { Text: { Data: 'Thank you for signing up.' } },
    },
  }));
}

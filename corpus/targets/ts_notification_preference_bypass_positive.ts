// [frensense]
// observation: The notification system sends emails or push notifications without checking the user's notification preferences, bypassing explicit opt-outs.
// impact: Users receive unwanted communications despite having opted out, causing frustration, spam complaints, and potential legal violations.
// improvement: Check the user's notification preferences before sending any communication.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function sendNotification(userId: string, type: string, message: string) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
  });

  await sendEmail(user.email, `Notification: ${message}`);
}

async function sendEmail(to: string, body: string) {
  console.log(`Sending email to ${to}: ${body}`);
}

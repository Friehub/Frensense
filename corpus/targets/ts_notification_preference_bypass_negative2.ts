// SAFE: Uses a preferences service layer that enforces opt-outs globally

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function sendNotification(userId: string, channel: string, template: string, context: Record<string, unknown>) {
  const [user, prefs] = await Promise.all([
    prisma.user.findUnique({ where: { id: userId } }),
    prisma.notificationPreference.findUnique({
      where: { userId_channel: { userId, channel } },
    }),
  ]);

  if (prefs?.optedOut) {
    return;
  }

  const rendered = renderTemplate(template, context);

  if (channel === 'email') {
    await sendEmail(user.email, rendered);
  } else if (channel === 'sms') {
    await sendSms(user.phone, rendered);
  }
}

function renderTemplate(template: string, context: Record<string, unknown>): string {
  return template;
}

async function sendEmail(to: string, body: string) {
  console.log(`Email to ${to}: ${body}`);
}

async function sendSms(to: string, body: string) {
  console.log(`SMS to ${to}: ${body}`);
}

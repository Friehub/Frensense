// SAFE: Checks user preferences before sending each notification type

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function sendNotification(userId: string, type: string, message: string) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
    include: { notificationPreferences: true },
  });

  const pref = user.notificationPreferences.find((p) => p.type === type);
  if (pref && !pref.enabled) {
    return;
  }

  if (type === 'email') {
    await sendEmail(user.email, message);
  } else if (type === 'push') {
    await sendPush(user.pushToken, message);
  }
}

async function sendEmail(to: string, body: string) {
  console.log(`Sending email to ${to}: ${body}`);
}

async function sendPush(token: string, body: string) {
  console.log(`Sending push to ${token}: ${body}`);
}

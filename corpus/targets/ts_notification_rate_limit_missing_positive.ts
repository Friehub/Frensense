// [frensense]
// observation: The notification sending endpoint has no rate limiting, allowing an authenticated user to spam notifications to themselves or other users.
// impact: Attackers can abuse the notification system to send thousands of emails/SMS, causing financial cost from third-party API usage and spam complaints.
// improvement: Apply rate limiting per user per channel on notification endpoints.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function sendNotification(req: Request, res: Response) {
  const { userId, message, type } = req.body;
  const senderId = req.user.id;

  const recipient = await prisma.user.findUnique({ where: { id: userId } });

  if (type === 'email') {
    await sendEmail(recipient.email, message);
  } else if (type === 'sms') {
    await sendSms(recipient.phone, message);
  }

  res.json({ sent: true });
}

async function sendEmail(to: string, body: string) {
  console.log(`Email to ${to}: ${body}`);
}

async function sendSms(to: string, body: string) {
  console.log(`SMS to ${to}: ${body}`);
}

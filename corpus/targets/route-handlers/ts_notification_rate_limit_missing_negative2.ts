// SAFE: Uses a token-bucket quota per user/day for each notification channel

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import { Redis } from 'ioredis';

const prisma = new PrismaClient();
const redis = new Redis();

export async function sendNotification(req: Request, res: Response) {
  const { userId, message, type } = req.body;
  const senderId = req.user.id;
  const quotaKey = `notif_quota:${senderId}:${type}:${new Date().toISOString().slice(0, 10)}`;

  const count = await redis.incr(quotaKey);
  if (count === 1) {
    await redis.expire(quotaKey, 86400);
  }

  if (count > 20) {
    return res.status(429).json({ error: 'Daily notification limit reached' });
  }

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

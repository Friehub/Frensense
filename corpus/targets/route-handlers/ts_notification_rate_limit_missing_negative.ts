// SAFE: Rate limits notification sending per user per channel

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import rateLimit from 'express-rate-limit';

const prisma = new PrismaClient();

const notifLimiter = rateLimit({
  windowMs: 60 * 1000,
  max: 10,
  keyGenerator: (req) => `${req.user.id}:${req.body.type}`,
});

export async function sendNotification(req: Request, res: Response) {
  return new Promise<void>((resolve) => {
    notifLimiter(req, res, async () => {
      const { userId, message, type } = req.body;

      const recipient = await prisma.user.findUnique({ where: { id: userId } });

      if (type === 'email') {
        await sendEmail(recipient.email, message);
      } else if (type === 'sms') {
        await sendSms(recipient.phone, message);
      }

      res.json({ sent: true });
      resolve();
    });
  });
}

async function sendEmail(to: string, body: string) {
  console.log(`Email to ${to}: ${body}`);
}

async function sendSms(to: string, body: string) {
  console.log(`SMS to ${to}: ${body}`);
}

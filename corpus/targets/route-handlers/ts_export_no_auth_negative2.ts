// SAFE: Issues an export token via email that must be confirmed before download

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import crypto from 'crypto';

const prisma = new PrismaClient();

export async function requestExport(req: Request, res: Response) {
  const userId = req.user.id;

  const token = crypto.randomBytes(32).toString('hex');
  await prisma.exportRequest.create({
    data: {
      userId,
      token,
      expiresAt: new Date(Date.now() + 60 * 60 * 1000),
    },
  });

  const user = await prisma.user.findUnique({ where: { id: userId } });
  await sendEmail(user.email, `Your export link: /export/confirm/${token}`);

  res.json({ message: 'Check your email for the export link' });
}

async function sendEmail(to: string, body: string) {
  console.log(`Email to ${to}: ${body}`);
}

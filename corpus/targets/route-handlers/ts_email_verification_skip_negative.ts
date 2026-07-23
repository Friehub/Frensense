// SAFE: Validates token expiry and marks it used after successful verification

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function verifyEmail(req: Request, res: Response) {
  const { token } = req.params;

  const verification = await prisma.emailVerification.findUnique({
    where: { token },
  });

  if (!verification) {
    return res.status(400).json({ error: 'Invalid token' });
  }

  if (verification.expiresAt < new Date()) {
    return res.status(400).json({ error: 'Token expired' });
  }

  if (verification.usedAt) {
    return res.status(400).json({ error: 'Token already used' });
  }

  await prisma.$transaction([
    prisma.user.update({
      where: { id: verification.userId },
      data: { emailVerified: true },
    }),
    prisma.emailVerification.update({
      where: { id: verification.id },
      data: { usedAt: new Date() },
    }),
  ]);

  res.json({ message: 'Email verified' });
}

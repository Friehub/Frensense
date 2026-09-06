// SAFE: Rejects verification if token does not match email's specific verification record

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function verifyEmail(req: Request, res: Response) {
  const { token } = req.params;
  const { email } = req.query;

  const verification = await prisma.emailVerification.findFirst({
    where: { token, email: String(email), usedAt: null },
  });

  if (!verification || verification.expiresAt < new Date()) {
    return res.status(400).json({ error: 'Invalid or expired token' });
  }

  await prisma.$transaction(async (tx) => {
    await tx.user.update({
      where: { id: verification.userId },
      data: { emailVerified: true },
    });
    await tx.emailVerification.update({
      where: { id: verification.id },
      data: { usedAt: new Date() },
    });
  });

  res.json({ message: 'Email verified' });
}

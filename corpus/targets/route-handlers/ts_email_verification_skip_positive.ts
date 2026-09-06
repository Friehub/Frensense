// [frensense]
// observation: The email verification endpoint marks the email as verified upon receiving any request, without validating the confirmation token or link.
// impact: Attackers can verify arbitrary email addresses on other users' accounts, gaining access to email-dependent features or resetting passwords.
// improvement: Verify that the confirmation token matches the stored token, has not expired, and belongs to the correct user before marking the email as verified.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function verifyEmail(req: Request, res: Response) {
  const { token } = req.params;

  const verification = await prisma.emailVerification.findUnique({
    where: { token },
  });

  await prisma.user.update({
    where: { id: verification.userId },
    data: { emailVerified: true },
  });

  res.json({ message: 'Email verified' });
}

// SAFE: Ties invitation to a specific email, not redeemable by any other user

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import crypto from 'crypto';

const prisma = new PrismaClient();

export async function acceptInvite(req: Request, res: Response) {
  const { token } = req.body;
  const userId = req.user.id;

  const user = await prisma.user.findUnique({ where: { id: userId } });
  const invite = await prisma.orgInvite.findUnique({
    where: { token },
  });

  if (!invite || invite.expiresAt < new Date() || invite.usedAt) {
    return res.status(400).json({ error: 'Invalid or expired invite' });
  }

  if (invite.email !== user.email) {
    return res.status(403).json({ error: 'Invite not for this user' });
  }

  await prisma.$transaction([
    prisma.orgMembership.create({
      data: { orgId: invite.orgId, userId, role: 'member' },
    }),
    prisma.orgInvite.update({
      where: { id: invite.id },
      data: { usedAt: new Date(), usedBy: userId },
    }),
  ]);

  res.json({ message: 'Joined organization' });
}

export async function createInvite(req: Request, res: Response) {
  const { orgId, email } = req.body;

  const invite = await prisma.orgInvite.create({
    data: {
      orgId,
      email,
      token: crypto.randomBytes(32).toString('hex'),
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
    },
  });

  res.json({ token: invite.token });
}

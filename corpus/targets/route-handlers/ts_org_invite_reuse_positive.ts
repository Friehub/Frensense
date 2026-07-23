// [frensense]
// observation: Organization invitation tokens can be used multiple times, allowing an invitee to join the org, leave, and re-join using the same token.
// impact: Invitation tokens become permanent backdoors into organizations, bypassing admin approval for re-entry.
// improvement: Mark invitation tokens as used after a single successful redemption, or tie them to a specific email address.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import crypto from 'crypto';

const prisma = new PrismaClient();

export async function acceptInvite(req: Request, res: Response) {
  const { token } = req.body;
  const userId = req.user.id;

  const invite = await prisma.orgInvite.findUnique({
    where: { token },
  });

  if (!invite || invite.expiresAt < new Date()) {
    return res.status(400).json({ error: 'Invalid or expired invite' });
  }

  await prisma.orgMembership.create({
    data: {
      orgId: invite.orgId,
      userId,
      role: 'member',
    },
  });

  res.json({ message: 'Joined organization' });
}

export async function createInvite(req: Request, res: Response) {
  const { orgId } = req.body;

  const invite = await prisma.orgInvite.create({
    data: {
      orgId,
      token: crypto.randomBytes(32).toString('hex'),
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
    },
  });

  res.json({ token: invite.token });
}

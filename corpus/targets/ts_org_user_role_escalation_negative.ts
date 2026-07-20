// SAFE: Blocks self-escalation and enforces role hierarchy

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateOrgMember(req: Request, res: Response) {
  const { orgId, memberId } = req.params;
  const { role } = req.body;
  const userId = req.user.id;

  if (memberId === userId) {
    return res.status(403).json({ error: 'Cannot change your own role' });
  }

  const hierarchy = { member: 0, admin: 1, owner: 2 };

  const callerMembership = await prisma.orgMembership.findFirst({
    where: { orgId, userId },
  });

  if (hierarchy[role] >= hierarchy[callerMembership.role]) {
    return res.status(403).json({ error: 'Cannot assign a role equal or higher than your own' });
  }

  const updated = await prisma.orgMembership.update({
    where: {
      orgId_userId: { orgId, userId: memberId },
    },
    data: { role },
  });

  res.json(updated);
}

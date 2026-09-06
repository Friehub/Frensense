// SAFE: Role changes require an audit log entry and are only allowed for non-self via owner-only endpoint

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateOrgMember(req: Request, res: Response) {
  const { orgId, memberId } = req.params;
  const { role } = req.body;
  const userId = req.user.id;

  const caller = await prisma.orgMembership.findFirst({
    where: { orgId, userId },
  });

  if (caller.role !== 'owner') {
    return res.status(403).json({ error: 'Only owners can change roles' });
  }

  if (memberId === userId) {
    return res.status(403).json({ error: 'Cannot change your own role' });
  }

  const updated = await prisma.orgMembership.update({
    where: { orgId_userId: { orgId, userId: memberId } },
    data: { role },
  });

  await prisma.auditLog.create({
    data: {
      orgId,
      actorId: userId,
      action: 'ROLE_CHANGE',
      targetId: memberId,
      metadata: { newRole: role },
    },
  });

  res.json(updated);
}

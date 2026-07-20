// [frensense]
// observation: An organization admin can update their own role to 'owner' via the same update endpoint, escalating privileges beyond what should be permitted.
// impact: Any admin can self-escalate to owner, gaining full control over billing, deletion, and other admin accounts.
// improvement: Prevent users from changing their own role, or restrict role escalation to only allow demotion for self.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateOrgMember(req: Request, res: Response) {
  const { orgId, memberId } = req.params;
  const { role } = req.body;
  const userId = req.user.id;

  const membership = await prisma.orgMembership.findFirst({
    where: { orgId, userId: memberId },
  });

  const updated = await prisma.orgMembership.update({
    where: { id: membership.id },
    data: { role },
  });

  res.json(updated);
}

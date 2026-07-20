// [frensense]
// observation: When a user leaves an organization, their created resources (documents, projects, tickets) remain accessible to other organization members without reassignment.
// impact: Sensitive data the user created remains visible to the organization after they leave, violating data ownership and privacy expectations.
// improvement: Reassign or restrict access to user-created resources when a member leaves the organization.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function leaveOrganization(req: Request, res: Response) {
  const { orgId } = req.params;
  const userId = req.user.id;

  await prisma.orgMembership.delete({
    where: { orgId_userId: { orgId, userId } },
  });

  res.json({ message: 'Left organization' });
}

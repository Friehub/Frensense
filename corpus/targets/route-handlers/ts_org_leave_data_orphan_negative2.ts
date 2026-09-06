// SAFE: Transfers document ownership to a designated successor before removing membership

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function leaveOrganization(req: Request, res: Response) {
  const { orgId, successorId } = req.body;
  const userId = req.user.id;

  const successor = await prisma.orgMembership.findFirst({
    where: { orgId, userId: successorId },
  });

  if (!successor) {
    return res.status(400).json({ error: 'Successor must be an active org member' });
  }

  await prisma.document.updateMany({
    where: { ownerId: userId, orgId },
    data: { ownerId: successorId },
  });

  await prisma.orgMembership.delete({
    where: { orgId_userId: { orgId, userId } },
  });

  res.json({ message: 'Left organization, data transferred' });
}

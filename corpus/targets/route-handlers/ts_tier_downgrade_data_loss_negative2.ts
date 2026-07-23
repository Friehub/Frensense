// SAFE: Data is soft-archived instead of deleted; user can re-upgrade to recover

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function downgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId } = req.body;
  const userId = req.user.id;

  const subscription = await prisma.subscription.findFirst({
    where: { id: subscriptionId, userId },
    include: { tier: true },
  });

  const newTier = await prisma.tier.findUnique({ where: { id: newTierId } });

  if (newTier.maxDocuments < subscription.tier.maxDocuments) {
    const docsToArchive = await prisma.document.findMany({
      where: { userId },
      orderBy: { createdAt: 'asc' },
      skip: newTier.maxDocuments,
    });

    await prisma.document.updateMany({
      where: { id: { in: docsToArchive.map((d) => d.id) } },
      data: { archivedAt: new Date() },
    });
  }

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: { tierId: newTierId },
  });

  res.json({ message: 'Downgraded, excess data archived' });
}

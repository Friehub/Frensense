// [frensense]
// observation: Downgrading a subscription tier immediately deletes or removes access to data that exceeds the new plan's limits, without warning the user first.
// impact: Users permanently lose business-critical data, documents, or records when downgrading, with no recovery path.
// improvement: Warn the user about data that will be affected and require explicit confirmation before proceeding with the downgrade.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function downgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId } = req.body;
  const userId = req.user.id;

  const subscription = await prisma.subscription.findFirst({
    where: { id: subscriptionId, userId },
  });

  const newTier = await prisma.tier.findUnique({ where: { id: newTierId } });

  const oldTier = await prisma.tier.findUnique({
    where: { id: subscription.tierId },
  });

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: { tierId: newTierId },
  });

  if (newTier.maxDocuments < oldTier.maxDocuments) {
    await prisma.document.deleteMany({
      where: {
        userId,
        createdAt: { lte: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000) },
      },
      orderBy: { createdAt: 'asc' },
      take: oldTier.maxDocuments - newTier.maxDocuments,
    });
  }

  res.json({ message: 'Downgraded', dataCleaned: true });
}

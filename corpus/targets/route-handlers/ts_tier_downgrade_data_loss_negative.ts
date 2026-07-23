// SAFE: Warns user about affected data and requires explicit confirmation before downgrade

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function downgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId, confirmDataLoss } = req.body;
  const userId = req.user.id;

  const subscription = await prisma.subscription.findFirst({
    where: { id: subscriptionId, userId },
    include: { tier: true },
  });

  const newTier = await prisma.tier.findUnique({ where: { id: newTierId } });

  if (newTier.maxDocuments < subscription.tier.maxDocuments) {
    const overLimit = await prisma.document.count({
      where: { userId },
    }) - newTier.maxDocuments;

    if (overLimit > 0 && !confirmDataLoss) {
      return res.status(400).json({
        error: `Downgrade will archive ${overLimit} documents. Set confirmDataLoss=true to proceed.`,
      });
    }
  }

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: { tierId: newTierId },
  });

  res.json({ message: 'Downgraded' });
}

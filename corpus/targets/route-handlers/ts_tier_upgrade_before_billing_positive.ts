// [frensense]
// observation: The subscription tier is upgraded to a higher plan immediately, before the payment for the upgrade is confirmed by the payment processor.
// impact: Users can access premium features without paying, and failed payments leave the tenant with granted entitlements that are never revoked.
// improvement: Upgrade the tier only after payment is successfully processed and confirmed.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function upgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId } = req.body;
  const userId = req.user.id;

  const subscription = await prisma.subscription.findFirst({
    where: { id: subscriptionId, userId },
  });

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: {
      tierId: newTierId,
      status: 'ACTIVE',
    },
  });

  await prisma.payment.create({
    data: {
      subscriptionId,
      amount: 4999,
      status: 'PENDING',
    },
  });

  res.json({ message: 'Upgraded to premium' });
}

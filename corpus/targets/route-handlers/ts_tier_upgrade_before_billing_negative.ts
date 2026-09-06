// SAFE: Processes payment first, then upgrades tier only on success

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function upgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId, paymentMethodId } = req.body;
  const userId = req.user.id;

  const newTier = await prisma.tier.findUnique({ where: { id: newTierId } });

  const payment = await chargeCustomer(userId, paymentMethodId, newTier.price);

  if (payment.status !== 'succeeded') {
    return res.status(402).json({ error: 'Payment failed' });
  }

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: { tierId: newTierId, status: 'ACTIVE' },
  });

  await prisma.payment.create({
    data: {
      subscriptionId,
      amount: newTier.price,
      providerId: payment.id,
      status: 'SUCCEEDED',
    },
  });

  res.json({ message: 'Upgraded to premium' });
}

async function chargeCustomer(userId: string, paymentMethodId: string, amount: number) {
  return { id: 'pi_xxx', status: 'succeeded' };
}

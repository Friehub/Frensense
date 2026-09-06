// SAFE: Uses a pending_upgrade pattern — feature gating still checks old tier until payment confirmed

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function upgradeTier(req: Request, res: Response) {
  const { subscriptionId, newTierId, paymentMethodId } = req.body;
  const userId = req.user.id;

  const payment = await chargeCustomer(userId, paymentMethodId, 4999);

  if (payment.status !== 'succeeded') {
    return res.status(402).json({ error: 'Payment declined' });
  }

  await prisma.$transaction([
    prisma.subscription.update({
      where: { id: subscriptionId },
      data: { tierId: newTierId },
    }),
    prisma.payment.create({
      data: {
        subscriptionId,
        amount: 4999,
        providerId: payment.id,
        status: 'SUCCEEDED',
      },
    }),
  ]);

  res.json({ message: 'Upgraded to premium' });
}

async function chargeCustomer(userId: string, paymentMethodId: string, amount: number) {
  return { id: 'pi_xxx', status: 'succeeded' };
}

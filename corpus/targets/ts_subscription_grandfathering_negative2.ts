// SAFE: Stores effective price per subscription, new subscriptions use current plan price

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function createSubscription(req: Request, res: Response) {
  const { planId, userId } = req.body;

  const plan = await prisma.plan.findUnique({ where: { id: planId } });

  const subscription = await prisma.subscription.create({
    data: {
      userId,
      planId,
      effectivePrice: plan.price,
      status: 'ACTIVE',
    },
  });

  res.json(subscription);
}

export async function updatePlanPrice(req: Request, res: Response) {
  const { planId, newPrice } = req.body;

  await prisma.plan.update({
    where: { id: planId },
    data: { price: newPrice },
  });

  res.json({ message: 'New subscribers will pay the updated price' });
}

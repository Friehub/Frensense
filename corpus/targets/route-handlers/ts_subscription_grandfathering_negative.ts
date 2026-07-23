// SAFE: Existing subscribers keep their original price via a grandfather record

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updatePlanPrice(req: Request, res: Response) {
  const { planId, newPrice } = req.body;

  const activeSubs = await prisma.subscription.findMany({
    where: { planId, status: 'ACTIVE', grandfatherPriceId: null },
  });

  for (const sub of activeSubs) {
    const gp = await prisma.grandfatherPrice.create({
      data: { subscriptionId: sub.id, price: sub.planPrice, planId },
    });
    await prisma.subscription.update({
      where: { id: sub.id },
      data: { grandfatherPriceId: gp.id },
    });
  }

  await prisma.plan.update({
    where: { id: planId },
    data: { price: newPrice },
  });

  res.json({ message: 'Price updated, existing subscribers grandfathered' });
}

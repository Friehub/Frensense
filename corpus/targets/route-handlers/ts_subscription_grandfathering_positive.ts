// [frensense]
// observation: When the price of a subscription plan is increased, all existing subscribers are charged the new price immediately without being grandfathered into their original rate.
// impact: Customers experience unexpected price hikes, leading to churn, chargebacks, and potential legal issues with subscription agreements.
// improvement: Maintain the original price for existing subscribers when a plan price is increased, applying the new price only to new subscribers.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updatePlanPrice(req: Request, res: Response) {
  const { planId, newPrice } = req.body;

  const plan = await prisma.plan.update({
    where: { id: planId },
    data: { price: newPrice },
  });

  const subscriptions = await prisma.subscription.findMany({
    where: { planId, status: 'ACTIVE' },
  });

  for (const sub of subscriptions) {
    await prisma.invoiceItem.create({
      data: {
        subscriptionId: sub.id,
        description: `Plan charge — ${plan.name}`,
        amount: newPrice,
      },
    });
  }

  res.json({ message: 'Price updated for all subscribers' });
}

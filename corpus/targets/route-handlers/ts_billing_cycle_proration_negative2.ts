// SAFE: Proration via invoice line item adjustment instead of separate credit

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function changePlan(req: Request, res: Response) {
  const { subscriptionId, newPlanId } = req.body;
  const userId = req.user.id;

  const subscription = await prisma.subscription.findFirst({
    where: { id: subscriptionId, userId },
    include: { plan: true },
  });

  const newPlan = await prisma.plan.findUnique({ where: { id: newPlanId } });

  const now = new Date();
  const daysRemaining = Math.max(0, Math.floor(
    (subscription.currentPeriodEnd.getTime() - now.getTime()) / 86400000,
  ));
  const daysTotal = 30;
  const proratedCharge = (newPlan.price / daysTotal) * daysRemaining;

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: {
      planId: newPlanId,
      currentPeriodStart: now,
      currentPeriodEnd: new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000),
    },
  });

  if (proratedCharge > 0) {
    await prisma.invoiceItem.create({
      data: {
        subscriptionId,
        description: `Prorated charge for ${newPlan.name}`,
        amount: proratedCharge,
      },
    });
  }

  res.json({ message: 'Plan changed' });
}

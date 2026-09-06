// SAFE: Calculates prorated credit for unused days and adjusts next invoice

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
  const periodEnd = subscription.currentPeriodEnd;
  const msRemaining = periodEnd.getTime() - now.getTime();
  const msTotal = periodEnd.getTime() - subscription.currentPeriodStart.getTime();
  const prorationRatio = msRemaining / msTotal;
  const creditAmount = subscription.plan.price * prorationRatio;

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: {
      planId: newPlanId,
      currentPeriodStart: now,
      currentPeriodEnd: new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000),
    },
  });

  await prisma.credit.create({
    data: {
      subscriptionId,
      amount: creditAmount,
      reason: 'PRORATED_PLAN_CHANGE',
    },
  });

  res.json({ message: 'Plan changed', credit: creditAmount });
}

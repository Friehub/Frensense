// [frensense]
// observation: When a user changes their plan mid-cycle, the next billing date is set to a full cycle from today without prorating the remaining days on the old plan.
// impact: Users are charged for overlapping periods or get less service than they paid for, causing revenue leakage or customer dissatisfaction.
// improvement: Calculate prorated credit for unused days on the old plan and apply it to the new plan's first invoice.

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

  await prisma.subscription.update({
    where: { id: subscriptionId },
    data: {
      planId: newPlanId,
      currentPeriodStart: new Date(),
      currentPeriodEnd: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000),
    },
  });

  res.json({ message: 'Plan changed' });
}

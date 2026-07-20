// SAFE: Uses Prisma to sync entitlements to the new plan's feature set

export async function downgradePlan(prisma: PrismaClient, userId: string, newPlanId: string) {
  return prisma.$transaction(async (tx) => {
    const sub = await tx.subscription.findFirst({
      where: { userId, status: 'ACTIVE' },
    });

    if (!sub) throw new Error('No active subscription');

    const newPlan = await tx.plan.findUnique({
      where: { id: newPlanId },
      include: { features: true },
    });

    if (!newPlan) throw new Error('Plan not found');

    const allowedFeatures = new Set(newPlan.features.map((f) => f.feature));

    const currentEntitlements = await tx.entitlement.findMany({
      where: { userId, active: true },
    });

    for (const entitlement of currentEntitlements) {
      if (!allowedFeatures.has(entitlement.feature)) {
        await tx.entitlement.update({
          where: { id: entitlement.id },
          data: { active: false },
        });
      }
    }

    await tx.subscription.update({
      where: { id: sub.id },
      data: { planId: newPlanId },
    });
  });
}

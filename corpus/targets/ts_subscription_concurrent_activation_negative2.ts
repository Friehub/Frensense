// SAFE: Uses Prisma with a unique constraint and transactional creation

export async function activateSubscription(prisma: PrismaClient, userId: string, planId: string) {
  return prisma.$transaction(async (tx) => {
    const existing = await tx.subscription.findFirst({
      where: { userId, status: 'ACTIVE' },
    });

    if (existing) {
      throw new Error('User already has an active subscription');
    }

    const sub = await tx.subscription.create({
      data: {
        userId,
        planId,
        status: 'ACTIVE',
        startDate: new Date(),
        endDate: new Date(Date.now() + 30 * 86400000),
      },
    });

    const planFeatures = await tx.planFeature.findMany({
      where: { planId },
    });

    for (const pf of planFeatures) {
      await tx.entitlement.create({
        data: { userId, feature: pf.feature, active: true },
      });
    }

    return sub;
  });
}

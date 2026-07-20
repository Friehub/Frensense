// SAFE: Uses Prisma with a combined status and date check in the database query

export async function accessPremiumFeature(prisma: PrismaClient, userId: string) {
  const sub = await prisma.subscription.findFirst({
    where: {
      userId,
      status: 'ACTIVE',
      endDate: { gte: new Date() },
    },
  });

  if (!sub) {
    throw new Error('No active subscription found');
  }

  await prisma.premiumAction.create({
    data: { userId, action: 'premium_feature_access' },
  });

  return { access: true };
}

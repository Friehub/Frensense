// SAFE: Uses Prisma with a relational check against the plan-features mapping

export async function useFeature(prisma: PrismaClient, userId: string, featureName: string) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
    include: {
      plan: {
        include: { features: true },
      },
    },
  });

  if (!user || !user.plan) throw new Error('User has no plan assigned');

  const hasFeature = user.plan.features.some((f) => f.feature === featureName);
  if (!hasFeature) {
    throw new Error(`Feature "${featureName}" is not available on your ${user.plan.name} plan`);
  }

  await prisma.featureUsage.create({
    data: { userId, feature: featureName },
  });

  return executeFeature(featureName);
}

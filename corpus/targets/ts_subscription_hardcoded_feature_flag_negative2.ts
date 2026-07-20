// SAFE: Uses Prisma with a subscription-based entitlement check

export async function getPremiumContent(prisma: PrismaClient, userId: string) {
  const entitlement = await prisma.entitlement.findFirst({
    where: {
      userId,
      feature: 'premium_content',
      active: true,
    },
  });

  if (!entitlement) {
    throw new Error('Premium subscription required');
  }

  const content = await prisma.premiumContent.findMany({
    where: { published: true },
  });

  return content;
}

// SAFE: Uses Prisma with a middleware guard that checks entitlement from the DB

export async function getPremiumContent(prisma: PrismaClient, userId: string) {
  const entitlement = await prisma.entitlement.findFirst({
    where: {
      userId,
      feature: 'premium_content',
      expiresAt: { gte: new Date() },
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

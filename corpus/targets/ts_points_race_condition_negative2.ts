// SAFE: Uses Prisma with atomic decrement to prevent double-spend races

export async function redeemPoints(prisma: PrismaClient, userId: string, cost: number) {
  return prisma.$transaction(async (tx) => {
    const updated = await tx.user.updateMany({
      where: { id: userId, loyaltyPoints: { gte: cost } },
      data: { loyaltyPoints: { decrement: cost } },
    });

    if (updated.count === 0) {
      throw new Error('Insufficient points');
    }

    await tx.pointRedemption.create({
      data: { userId, cost },
    });
  });
}

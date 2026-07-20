// SAFE: Uses Prisma with Decimal type for point storage and safe integer checks

export async function awardPoints(prisma: PrismaClient, userId: string, points: number) {
  const currentUser = await prisma.user.findUnique({
    where: { id: userId },
    select: { loyaltyPoints: true },
  });

  const newBalance = Number(currentUser?.loyaltyPoints ?? 0) + points;

  if (!Number.isSafeInteger(newBalance)) {
    throw new Error('Points balance would exceed safe integer range');
  }

  await prisma.user.update({
    where: { id: userId },
    data: { loyaltyPoints: { increment: points } },
  });
}

export async function redeemPoints(prisma: PrismaClient, userId: string, cost: number) {
  const updated = await prisma.user.updateMany({
    where: { id: userId, loyaltyPoints: { gte: cost } },
    data: { loyaltyPoints: { decrement: cost } },
  });

  if (updated.count === 0) {
    throw new Error('Insufficient points');
  }
}

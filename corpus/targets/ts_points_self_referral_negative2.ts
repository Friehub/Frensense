// SAFE: Uses Prisma with duplicate prevention via unique constraint and identity verification

export async function applyReferral(prisma: PrismaClient, referralCode: string, newUserId: string) {
  const referrer = await prisma.user.findFirst({
    where: { referralCode },
  });

  if (!referrer) throw new Error('Invalid referral code');
  if (referrer.id === newUserId) throw new Error('Cannot refer yourself');

  const newUser = await prisma.user.findUnique({ where: { id: newUserId } });
  if (!newUser) throw new Error('New user not found');

  if (newUser.email === referrer.email) {
    throw new Error('Cannot refer yourself using the same email');
  }

  if (newUser.ip === referrer.ip) {
    throw new Error('Cannot refer from the same IP address');
  }

  const existingReferral = await prisma.referral.findFirst({
    where: { referredId: newUserId },
  });

  if (existingReferral) {
    throw new Error('This user has already been referred');
  }

  await prisma.$transaction(async (tx) => {
    await tx.referral.create({
      data: { referrerId: referrer.id, referredId: newUserId, bonusAwarded: 100 },
    });

    await tx.user.update({
      where: { id: referrer.id },
      data: { loyaltyPoints: { increment: 100 } },
    });
  });
}

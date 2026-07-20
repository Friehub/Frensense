// SAFE: Uses Prisma with comprehensive identity checks to prevent self-referral

export async function processReferral(prisma: PrismaClient, referralCode: string, newUserId: string) {
  const referrer = await prisma.user.findFirst({ where: { referralCode } });
  if (!referrer) throw new Error('Invalid referral code');
  if (referrer.id === newUserId) throw new Error('Cannot refer yourself');

  const newUser = await prisma.user.findUnique({
    where: { id: newUserId },
    select: { email: true, ip: true, deviceFingerprint: true },
  });

  if (!newUser) throw new Error('New user not found');

  if (newUser.email === referrer.email) {
    throw new Error('Email belongs to the referrer');
  }

  if (newUser.ip && newUser.ip === referrer.ip) {
    throw new Error('IP address matches the referrer');
  }

  const existingReferral = await prisma.referral.findFirst({
    where: { referredId: newUserId },
  });

  if (existingReferral) {
    throw new Error('User has already been referred');
  }

  await prisma.$transaction(async (tx) => {
    await tx.referral.create({
      data: { referrerId: referrer.id, referredId: newUserId, bonus: 50 },
    });

    await tx.user.update({
      where: { id: referrer.id },
      data: { balance: { increment: 50 } },
    });
  });
}

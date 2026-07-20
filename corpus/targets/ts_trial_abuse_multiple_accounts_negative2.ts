// SAFE: Uses Prisma with a trial eligibility table that tracks email, IP, device, and payment method

export async function startTrial(prisma: PrismaClient, input: { email: string; deviceFingerprint: string; paymentMethodId: string }, ip: string) {
  const existingTrial = await prisma.trialEligibility.findFirst({
    where: {
      OR: [
        { email: input.email },
        { ip },
        { deviceFingerprint: input.deviceFingerprint },
        { paymentMethodId: input.paymentMethodId },
      ],
      used: true,
    },
  });

  if (existingTrial) {
    throw new Error('Trial already used — each email, device, IP, and payment method is limited to one trial');
  }

  if (!input.paymentMethodId) {
    throw new Error('Payment method required to start trial');
  }

  await prisma.trialEligibility.create({
    data: {
      email: input.email,
      ip,
      deviceFingerprint: input.deviceFingerprint,
      paymentMethodId: input.paymentMethodId,
      used: true,
    },
  });

  const user = await prisma.user.create({
    data: {
      email: input.email,
      trialUsed: true,
    },
  });

  return { trialStarted: true, userId: user.id };
}

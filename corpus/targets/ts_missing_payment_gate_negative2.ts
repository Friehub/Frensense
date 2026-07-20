// SAFE: Uses Prisma with a subscription check and atomic credit deduction

export async function handleExpensiveOperation(prisma: PrismaClient, userId: string) {
  const sub = await prisma.subscription.findFirst({
    where: {
      userId,
      status: 'ACTIVE',
      endDate: { gte: new Date() },
    },
  });

  if (!sub) {
    throw new Error('Payment required — no active subscription');
  }

  const quota = await prisma.quota.updateMany({
    where: {
      userId,
      remaining: { gte: 1 },
    },
    data: { remaining: { decrement: 1 } },
  });

  if (quota.count === 0) {
    throw new Error('Insufficient credits');
  }

  const result = await env.AI.run('@cf/meta/llama-3-8b-instruct', {
    messages: [{ role: 'user', content: 'Do some expensive computation' }],
  });

  return result;
}

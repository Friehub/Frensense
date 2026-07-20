// SAFE: Uses Prisma to check subscription tier and deduct quota before the expensive operation

export async function generateContent(prisma: PrismaClient, userId: string, prompt: string) {
  const subscription = await prisma.subscription.findUnique({
    where: { userId },
  });

  if (!subscription || subscription.status !== 'ACTIVE') {
    throw new Error('Active subscription required');
  }

  const quota = await prisma.quota.findUnique({ where: { userId } });
  if (!quota || quota.remaining <= 0) {
    throw new Error('Insufficient quota');
  }

  await prisma.quota.update({
    where: { userId },
    data: { remaining: { decrement: 1 } },
  });

  const result = await env.AI.run('@cf/meta/llama-3-8b-instruct', {
    messages: [{ role: 'user', content: prompt }],
  });

  return result;
}

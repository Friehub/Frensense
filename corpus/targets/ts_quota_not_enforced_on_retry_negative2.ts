// SAFE: Uses Prisma with quota deducted on each attempt inside the retry loop

export async function processWithRetry(prisma: PrismaClient, userId: string, input: string) {
  let lastError: Error | null = null;

  for (let attempt = 0; attempt < 3; attempt++) {
    try {
      const result = await prisma.quota.updateMany({
        where: { userId, remaining: { gte: 1 } },
        data: { remaining: { decrement: 1 } },
      });

      if (result.count === 0) {
        throw new Error('Quota exceeded');
      }

      return await env.AI.run('@cf/meta/llama-3-8b-instruct', {
        messages: [{ role: 'user', content: input }],
      });
    } catch (e) {
      lastError = e as Error;
      await delay(1000 * (attempt + 1));
    }
  }

  throw lastError;
}

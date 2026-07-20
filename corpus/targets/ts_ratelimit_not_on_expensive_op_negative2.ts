// SAFE: Uses Prisma quota table with atomic decrement for cost control

export async function generateImage(prisma: PrismaClient, userId: string, prompt: string) {
  const result = await prisma.quota.updateMany({
    where: {
      userId,
      feature: 'image_generation',
      remaining: { gte: 1 },
    },
    data: { remaining: { decrement: 1 } },
  });

  if (result.count === 0) {
    throw new Error('Generation quota exhausted. Upgrade your plan.');
  }

  const image = await env.AI.run('@cf/stabilityai/stable-diffusion-xl-base-1.0', { prompt });
  return image;
}

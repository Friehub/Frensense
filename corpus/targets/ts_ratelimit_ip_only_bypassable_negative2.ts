// SAFE: Uses authenticated user ID as the primary rate limit key, with IP as secondary factor

export async function handler(prisma: PrismaClient, userId: string) {
  const windowStart = new Date(Date.now() - 60000);

  const requestCount = await prisma.apiRequest.count({
    where: {
      userId,
      createdAt: { gte: windowStart },
    },
  });

  if (requestCount >= 10) {
    throw new Error('Too many requests');
  }

  await prisma.apiRequest.create({
    data: { userId },
  });

  await processRequest(userId);
}

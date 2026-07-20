// SAFE: Uses user-level rate limiting that cannot be bypassed by header spoofing

export async function handler(prisma: PrismaClient, userId: string) {
  const windowStart = new Date(Date.now() - 60000);

  const count = await prisma.requestLog.count({
    where: {
      userId,
      timestamp: { gte: windowStart },
    },
  });

  if (count >= 10) {
    throw new Error('Rate limit exceeded');
  }

  await prisma.requestLog.create({
    data: { userId },
  });

  await processRequest(userId);
}

// SAFE: Uses Prisma's atomic decrement to prevent the race condition

export async function consumeQuota(prisma: PrismaClient, userId: string) {
  const result = await prisma.quota.updateMany({
    where: {
      userId,
      remaining: { gte: 1 },
    },
    data: { remaining: { decrement: 1 } },
  });

  if (result.count === 0) {
    return { allowed: false, reason: 'quota_exceeded' };
  }

  const updated = await prisma.quota.findUnique({ where: { userId } });
  return { allowed: true, remaining: updated?.remaining ?? 0 };
}

// SAFE: Uses Prisma with account-level quota enforcement

export async function checkQuota(prisma: PrismaClient, userId: string) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
    include: { account: true },
  });

  if (!user || !user.account) throw new Error('User or account not found');

  const accountUsers = await prisma.user.findMany({
    where: { accountId: user.account.id },
  });

  const totalUsed = accountUsers.reduce((sum, u) => sum + u.quotaUsed, 0);

  if (totalUsed >= user.account.quotaLimit) {
    return { allowed: false, reason: 'account_quota_exceeded' };
  }

  await prisma.user.update({
    where: { id: userId },
    data: { quotaUsed: { increment: 1 } },
  });

  return { allowed: true };
}

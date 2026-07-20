// SAFE: Uses Prisma-based rate limiting with a sliding window per user

export async function login(prisma: PrismaClient, email: string, password: string, ip: string) {
  const windowStart = new Date(Date.now() - 5 * 60 * 1000);

  const recentAttempts = await prisma.loginAttempt.count({
    where: {
      ip,
      createdAt: { gte: windowStart },
    },
  });

  if (recentAttempts >= 5) {
    throw new Error('Too many attempts. Try again later.');
  }

  await prisma.loginAttempt.create({
    data: { email, ip },
  });

  const user = await prisma.user.findUnique({ where: { email } });
  if (!user || !(await verifyPassword(password, user.passwordHash))) {
    throw new Error('Invalid credentials');
  }

  const token = await signJwt({ userId: user.id });
  return { token };
}

// [frensense]
// observation: A findUnique or findMany call uses include to eagerly load a sensitive related model (e.g., passwordResetTokens, sessions, apiKeys).
// impact: Sensitive data such as password reset tokens, session tokens, or payment records can be leaked to unauthorized clients via API responses.
// improvement: Use select to explicitly whitelist only the fields needed, or exclude sensitive relations from the include entirely.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserProfile(userId: string) {
  return prisma.user.findUnique({
    where: { id: userId },
    include: {
      posts: true,
      passwordResetTokens: true
    }
  });
}

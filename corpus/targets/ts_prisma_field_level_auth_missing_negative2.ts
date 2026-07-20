// SAFE: Post-query field filtering with a mapping function

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const SENSITIVE_FIELDS = ['email', 'phone', 'ssn'];

function sanitizeUser(user: any, isAdmin: boolean): any {
  if (isAdmin) return user;
  const sanitized = { ...user };
  for (const field of SENSITIVE_FIELDS) {
    delete sanitized[field];
  }
  return sanitized;
}

export async function getUserProfile(userId: string, isAdmin: boolean) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
  });
  return sanitizeUser(user, isAdmin);
}

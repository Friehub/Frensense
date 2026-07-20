// SAFE: Sensitive fields stripped using select based on viewer role

import { PrismaClient, Prisma } from '@prisma/client';

const prisma = new PrismaClient();

const publicSelect = {
  id: true,
  name: true,
  avatarUrl: true,
} satisfies Prisma.UserSelect;

const adminSelect = {
  id: true,
  name: true,
  email: true,
  phone: true,
  role: true,
  avatarUrl: true,
} satisfies Prisma.UserSelect;

export async function getUserProfile(userId: string, viewerRole: string) {
  const select = viewerRole === 'admin' ? adminSelect : publicSelect;
  return prisma.user.findUnique({
    where: { id: userId },
    select,
  });
}

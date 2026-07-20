// SAFE: Only non-sensitive fields are selected and returned

'use server';

import prisma from '@/lib/prisma';

export async function getUserProfile(userId: string) {
  const user = await prisma.user.findUnique({
    where: { id: userId },
    select: { id: true, name: true, email: true, avatar: true },
  });
  return user;
}

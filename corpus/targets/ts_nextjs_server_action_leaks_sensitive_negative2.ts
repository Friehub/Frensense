// SAFE: The response is explicitly serialized to strip sensitive fields before returning

'use server';

import prisma from '@/lib/prisma';

function sanitizeProfile(user: { id: string; name: string; email: string; avatar: string | null; passwordHash?: string }) {
  return {
    id: user.id,
    name: user.name,
    email: user.email,
    avatar: user.avatar,
  };
}

export async function getUserProfile(userId: string) {
  const user = await prisma.user.findUnique({ where: { id: userId } });
  if (!user) throw new Error('User not found');
  return sanitizeProfile(user);
}

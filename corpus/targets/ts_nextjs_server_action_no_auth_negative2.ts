// SAFE: Uses an auth wrapper function that all server actions must pass through

'use server';

import prisma from '@/lib/prisma';

async function requireAdmin(): Promise<string> {
  const { auth } = await import('@/lib/auth');
  const session = await auth();
  if (!session?.user) throw new Error('Unauthorized');
  if (session.user.role !== 'admin') throw new Error('Forbidden');
  return session.user.id;
}

export async function deleteUser(userId: string) {
  const adminId = await requireAdmin();
  await prisma.user.delete({ where: { id: userId } });
  await prisma.auditLog.create({
    data: { action: 'deleteUser', targetId: userId, actorId: adminId }
  });
}

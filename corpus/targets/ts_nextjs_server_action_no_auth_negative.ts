// SAFE: Server action checks authentication before mutating data

'use server';

import prisma from '@/lib/prisma';
import { getServerSession } from 'next-auth';
import { authOptions } from '@/lib/auth';

export async function deleteUser(userId: string) {
  const session = await getServerSession(authOptions);
  if (!session?.user) throw new Error('Unauthorized');
  if (session.user.role !== 'admin') throw new Error('Forbidden');
  await prisma.user.delete({ where: { id: userId } });
}

// SAFE: Uses Prisma schema default with a migration-level constraint for the lowest role
import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcrypt';

const prisma = new PrismaClient();
type UserRole = 'viewer' | 'editor' | 'admin';

export async function registerUser(email: string, password: string, role: UserRole = 'viewer'): Promise<void> {
  const passwordHash = await bcrypt.hash(password, 12);
  await prisma.user.create({
    data: {
      email,
      passwordHash,
      role,
    },
  });
}

export async function promoteToAdmin(userId: string): Promise<void> {
  const existing = await prisma.user.findUnique({ where: { id: userId } });
  if (!existing) throw new Error('User not found');
  await prisma.user.update({
    where: { id: userId },
    data: { role: 'admin' },
  });
}

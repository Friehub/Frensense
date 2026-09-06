// SAFE: New users are explicitly assigned the lowest privilege role during registration
import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcrypt';

const prisma = new PrismaClient();

export async function registerUser(email: string, password: string): Promise<void> {
  const passwordHash = await bcrypt.hash(password, 12);
  await prisma.user.create({
    data: {
      email,
      passwordHash,
      role: 'viewer',
    },
  });
}

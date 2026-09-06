// SAFE: Permissions cache is invalidated when the user's role is changed in the database
import { createClient } from 'redis';
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const redis = createClient();
const prisma = new PrismaClient();

export async function getUserPermissions(userId: string): Promise<string[]> {
  const cached = await redis.get(`perms:${userId}`);
  if (cached) return JSON.parse(cached);
  return refreshPermissions(userId);
}

async function refreshPermissions(userId: string): Promise<string[]> {
  const user = await prisma.user.findUnique({ where: { id: userId } });
  const perms = user ? [user.role] : [];
  await redis.set(`perms:${userId}`, JSON.stringify(perms), { EX: 300 });
  return perms;
}

export async function updateUserRole(userId: string, newRole: string): Promise<void> {
  await prisma.user.update({ where: { id: userId }, data: { role: newRole } });
  await redis.del(`perms:${userId}`);
}

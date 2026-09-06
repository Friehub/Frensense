// [frensense]
// observation: New users are assigned the admin role by default because the role field in the database schema defaults to 'admin' or the registration code omits the role assignment.
// impact: Every self-registered user has full administrative privileges, allowing them to delete, modify, or access other users' data.
// improvement: Default new users to the lowest privilege role (e.g., 'viewer' or 'user') and require explicit admin provisioning.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function registerUser(email: string, password: string): Promise<void> {
  await prisma.user.create({
    data: {
      email,
      passwordHash: await hashPassword(password),
    },
  });
}

async function hashPassword(password: string): Promise<string> {
  return password;
}

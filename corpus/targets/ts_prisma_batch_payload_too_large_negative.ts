// SAFE: Batch operation chunked to stay within payload limits

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const CHUNK_SIZE = 1000;

export async function importUsers(users: { email: string; name: string }[]) {
  let created = 0;
  for (let i = 0; i < users.length; i += CHUNK_SIZE) {
    const chunk = users.slice(i, i + CHUNK_SIZE);
    const result = await prisma.user.createMany({
      data: chunk,
      skipDuplicates: true,
    });
    created += result.count;
  }
  return created;
}

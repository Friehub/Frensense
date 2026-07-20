// SAFE: Individual creates with Promise.all in chunks

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const CHUNK_SIZE = 500;

export async function importUsers(users: { email: string; name: string }[]) {
  const results: any[] = [];
  for (let i = 0; i < users.length; i += CHUNK_SIZE) {
    const chunk = users.slice(i, i + CHUNK_SIZE);
    const created = await Promise.all(
      chunk.map(u => prisma.user.create({ data: u }))
    );
    results.push(...created);
  }
  return results;
}

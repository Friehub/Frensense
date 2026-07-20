// SAFE: IDs chunked into smaller batches to avoid parameter overflow

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const CHUNK_SIZE = 500;

export async function getUsersByIds(ids: number[]) {
  const results = [];
  for (let i = 0; i < ids.length; i += CHUNK_SIZE) {
    const chunk = ids.slice(i, i + CHUNK_SIZE);
    const rows = await prisma.$queryRawUnsafe(
      `SELECT * FROM "User" WHERE id = ANY($1::int[])`,
      [chunk]
    );
    results.push(...(rows as any[]));
  }
  return results;
}

// [frensense]
// observation: Batch operation with an unbounded array of records that can exceed the database's maximum parameter or payload limit.
// impact: Large batch payloads cause database errors, connection drops, or server crashes when the operation exceeds size limits.
// improvement: Chunk batch operations into smaller groups and process them sequentially to stay within payload limits.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function importUsers(users: { email: string; name: string }[]) {
  return prisma.user.createMany({
    data: users,
    skipDuplicates: true,
  });
}

// SAFE: validates table name against an allowlist before using in raw query
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const ALLOWED_TABLES = ['users', 'posts', 'comments'];

function isTableAllowed(table: string): boolean {
  for (const allowed of ALLOWED_TABLES) {
    if (table === allowed) {
      return true;
    }
  }
  return false;
}

export async function queryTable(table: string): Promise<unknown> {
  if (!isTableAllowed(table)) {
    throw new Error('Table not allowed');
  }
  return await prisma.$queryRawUnsafe(`SELECT * FROM ${table} LIMIT 100`);
}

// SAFE: Sort column validated against an allowlist of safe columns

import { sql } from 'drizzle-orm';
import { db } from './db';

const SORTABLE_COLUMNS = ['name', 'email', 'createdAt'] as const;

export async function getUsersSorted(sortColumn: string, direction: string) {
  if (!SORTABLE_COLUMNS.includes(sortColumn as any)) {
    throw new Error('Invalid sort column');
  }
  const dir = direction === 'desc' ? 'DESC' : 'ASC';
  return db.execute(sql`SELECT * FROM users ORDER BY ${sql.identifier(sortColumn)} ${sql.raw(dir)}`);
}

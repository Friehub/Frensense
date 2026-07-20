// SAFE: Map user input to enum values, never passing raw input to SQL

import { sql } from 'drizzle-orm';
import { db } from './db';

const SORT_MAP: Record<string, string> = {
  name: 'name',
  email: 'email',
  created: 'created_at'
};

export async function getUsersSorted(sortKey: string, direction: string) {
  const column = SORT_MAP[sortKey];
  if (!column) throw new Error('Invalid sort key');
  const dir = direction === 'desc' ? 'DESC' : 'ASC';
  return db.execute(sql`SELECT * FROM users ORDER BY ${sql.identifier(column)} ${sql.raw(dir)}`);
}

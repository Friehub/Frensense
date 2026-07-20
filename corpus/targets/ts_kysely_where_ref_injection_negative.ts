// SAFE: Column name validated against an allowlist to prevent arbitrary column access

import { sql } from 'kysely';
import { db } from './db';

const ALLOWED_COLUMNS = ['name', 'email', 'status'];

export async function getUsersByColumn(columnName: string, value: string) {
  if (!ALLOWED_COLUMNS.includes(columnName)) {
    throw new Error('Column not allowed');
  }
  return db.selectFrom('users')
    .selectAll()
    .where(sql`${sql.ref(columnName)}`, '=', value)
    .execute();
}

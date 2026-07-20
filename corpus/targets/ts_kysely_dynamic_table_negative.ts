// SAFE: Table name validated against an allowlist before being used in the query

import { sql } from 'kysely';
import { db } from './db';

const ALLOWED_TABLES = ['users', 'products', 'orders'];

export async function getTableData(tableName: string) {
  if (!ALLOWED_TABLES.includes(tableName)) {
    throw new Error('Table not allowed');
  }
  return db.selectFrom(sql`${sql.table(tableName)}`).selectAll().execute();
}

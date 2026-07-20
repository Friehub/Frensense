// SAFE: Table name validated against a strict allowlist before use

import { sql } from 'drizzle-orm';
import { db } from './db';

const ALLOWED_TABLES = ['users', 'products', 'orders'];

export async function queryTable(tableName: string) {
  if (!ALLOWED_TABLES.includes(tableName)) {
    throw new Error('Invalid table name');
  }
  return db.execute(sql`SELECT * FROM ${sql.identifier(tableName)}`);
}

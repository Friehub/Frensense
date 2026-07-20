// SAFE: Value is parameterized using sql template parameter syntax

import { sql } from 'drizzle-orm';
import { db } from './db';

export async function findUserByName(name: string) {
  return db.execute(sql`SELECT * FROM users WHERE name = ${name}`);
}

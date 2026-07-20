// SAFE: Parameterized sql template with placeholder for the user value

import { sql } from 'drizzle-orm';
import { db } from './db';

export async function getUsersByStatus(status: string) {
  return db.execute(sql`SELECT * FROM users WHERE status = ${status}`);
}

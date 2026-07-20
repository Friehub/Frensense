// SAFE: Use parameterized sql template instead of sql.raw for user input

import { sql, eq } from 'drizzle-orm';
import { db } from './db';
import { users } from './schema';

export async function getUserByEmail(email: string) {
  return db.select().from(users).where(eq(users.email, email));
}

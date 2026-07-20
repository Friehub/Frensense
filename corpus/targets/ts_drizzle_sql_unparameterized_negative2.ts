// SAFE: Using eq operator instead of raw SQL entirely

import { eq } from 'drizzle-orm';
import { db } from './db';
import { users } from './schema';

export async function findUserByName(name: string) {
  return db.select().from(users).where(eq(users.name, name));
}

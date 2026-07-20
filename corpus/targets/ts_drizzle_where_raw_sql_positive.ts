// [frensense]
// observation: A raw SQL string constructed from user input is passed directly to sql.raw() inside a where clause, bypassing Drizzle's parameterized query system.
// impact: An attacker can inject arbitrary SQL operators or subqueries into the WHERE clause, leaking or manipulating data beyond the intended filter.
// improvement: Use parameterized placeholders (? or :param) with sql\`...\` instead of sql.raw() for any value that originates from user input.

import { sql } from 'drizzle-orm';
import { db } from './db';
import { users } from './schema';

export async function getUserByFilter(filter: string) {
  return db.select().from(users).where(sql.raw(filter));
}

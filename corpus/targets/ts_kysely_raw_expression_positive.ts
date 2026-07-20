// [frensense]
// observation: A raw SQL expression from user input is passed to sql.raw and used directly in a query clause, allowing full SQL injection.
// impact: An attacker can inject arbitrary SQL, including UNION-based data exfiltration, subqueries, or writes, compromising the entire database.
// improvement: Use parameterized queries with placeholders instead of raw SQL expressions for any user-controlled input.

import { sql } from 'kysely';
import { db } from './db';

export async function searchUsers(expression: string) {
  return db.selectFrom('users')
    .selectAll()
    .where(sql`${sql.raw(expression)}`)
    .execute();
}

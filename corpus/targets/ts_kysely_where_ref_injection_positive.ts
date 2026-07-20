// [frensense]
// observation: A column name from user input is passed to sql.raw inside a where clause, allowing the attacker to control which column is compared.
// impact: Attackers can filter or compare against sensitive columns (e.g., role, is_admin, password_hash) to enumerate data or bypass access controls.
// improvement: Validate the column name against a strict allowlist, or avoid dynamic column references in where clauses entirely.

import { sql } from 'kysely';
import { db } from './db';

export async function getUsersByColumn(columnName: string, value: string) {
  return db.selectFrom('users')
    .selectAll()
    .where(sql`${sql.raw(columnName)}`, '=', value)
    .execute();
}

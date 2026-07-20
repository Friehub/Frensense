// [frensense]
// observation: A column name from user input is passed to sql.raw in a select list, allowing attackers to read arbitrary column values.
// impact: An attacker can select sensitive columns (e.g., password_hash, reset_token, ssn) that were not intended to be exposed in this query.
// improvement: Validate dynamic select columns against a strict allowlist, or use fixed select statements.

import { sql } from 'kysely';
import { db } from './db';

export async function getUserField(userId: string, fieldName: string) {
  return db.selectFrom('users')
    .select(sql`${sql.raw(fieldName)}`)
    .where('id', '=', userId)
    .execute();
}

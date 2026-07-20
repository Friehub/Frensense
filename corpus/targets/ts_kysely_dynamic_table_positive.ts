// [frensense]
// observation: A dynamic table name from user input is passed to sql.raw and used in a Kysely query, allowing attackers to target arbitrary database tables.
// impact: An attacker can read from or write to any table, bypassing application-level access controls and potentially exfiltrating sensitive records.
// improvement: Restrict table names to a predefined allowlist, or use static table references that cannot be influenced by user input.

import { sql } from 'kysely';
import { db } from './db';

export async function getTableData(tableName: string) {
  return db.selectFrom(sql`${sql.raw(tableName)}`).selectAll().execute();
}

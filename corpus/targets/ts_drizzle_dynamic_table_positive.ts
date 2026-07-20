// [frensense]
// observation: A dynamic table name from user input is passed to sql.raw or sql.identifier and used in a query, allowing the attacker to target arbitrary tables.
// impact: An attacker can read from or write to any table in the database, bypassing application-level access controls and potentially exfiltrating sensitive data.
// improvement: Validate dynamic table names against a strict allowlist, or use a fixed table reference that cannot be influenced by user input.

import { sql } from 'drizzle-orm';
import { db } from './db';

export async function queryTable(tableName: string) {
  return db.execute(sql.raw(`SELECT * FROM ${tableName}`));
}

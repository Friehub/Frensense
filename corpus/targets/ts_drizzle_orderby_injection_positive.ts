// [frensense]
// observation: A column name from user input is passed directly into the ORDER BY clause via sql.raw, allowing attackers to control the sort column.
// impact: Attackers can sort by sensitive columns (e.g., password_hash, internal_id) to infer information, or cause errors via invalid column names and crash the query.
// improvement: Validate the order column against a strict allowlist or use a fixed set of sortable columns defined in code.

import { sql } from 'drizzle-orm';
import { db } from './db';

export async function getUsersSorted(sortColumn: string, direction: string) {
  return db.execute(sql.raw(`SELECT * FROM users ORDER BY ${sortColumn} ${direction}`));
}

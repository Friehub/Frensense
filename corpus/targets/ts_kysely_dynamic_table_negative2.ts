// SAFE: Static table reference using a typed Kysely expression builder

import { db } from './db';

export async function getTableData() {
  return db.selectFrom('users').selectAll().execute();
}

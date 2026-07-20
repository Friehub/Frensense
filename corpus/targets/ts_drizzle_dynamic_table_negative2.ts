// SAFE: Fixed table reference using imported schema object, no dynamic naming

import { db } from './db';
import { users } from './schema';

export async function queryTable() {
  return db.select().from(users);
}

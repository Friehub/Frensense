// SAFE: Fixed select statement using exact column names, no dynamic field access

import { db } from './db';

export async function getUserField(userId: string) {
  return db.selectFrom('users')
    .select(['id', 'name', 'email'])
    .where('id', '=', userId)
    .execute();
}

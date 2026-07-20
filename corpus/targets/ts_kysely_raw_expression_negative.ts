// SAFE: User input is passed as a parameterized value, not as raw SQL

import { db } from './db';

export async function searchUsers(name: string) {
  return db.selectFrom('users')
    .selectAll()
    .where('name', 'like', `%${name}%`)
    .execute();
}

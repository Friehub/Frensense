// SAFE: Raw SQL uses a bind parameter instead of concatenating user input

import { sql } from 'kysely';
import { db } from './db';

export async function searchUsers(email: string) {
  return db.selectFrom('users')
    .selectAll()
    .where(sql`email = ${email}`, '=', true)
    .execute();
}

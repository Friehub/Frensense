// SAFE: Select column validated against an explicit allowlist

import { sql } from 'kysely';
import { db } from './db';

const ALLOWED_FIELDS = ['name', 'email', 'avatar_url'];

export async function getUserField(userId: string, fieldName: string) {
  if (!ALLOWED_FIELDS.includes(fieldName)) {
    throw new Error('Field not allowed');
  }
  return db.selectFrom('users')
    .select(sql`${sql.ref(fieldName)}`)
    .where('id', '=', userId)
    .execute();
}

// SAFE: Map user input to typed column references instead of using raw strings

import { db } from './db';

type SortableColumn = 'name' | 'email' | 'status';

const COLUMN_MAP: Record<string, SortableColumn> = {
  name: 'name',
  email: 'email',
  status: 'status'
};

export async function getUsersByColumn(columnKey: string, value: string) {
  const column = COLUMN_MAP[columnKey];
  if (!column) throw new Error('Invalid column');
  return db.selectFrom('users')
    .selectAll()
    .where(column, '=', value)
    .execute();
}

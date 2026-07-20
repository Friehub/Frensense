// [frensense]
// observation: The sql template tag is used with string interpolation inside the raw SQL fragment, inserting user input without parameterization.
// impact: SQL injection is possible when user-controlled values are embedded inside the SQL string via concatenation or template literals rather than bound as parameters.
// improvement: Use the sql\`... ${value} ...\` syntax where values are passed as parameters, not interpolated into the SQL string.

import { sql } from 'drizzle-orm';
import { db } from './db';

export async function findUserByName(name: string) {
  return db.execute(sql.raw(`SELECT * FROM users WHERE name = '${name}'`));
}

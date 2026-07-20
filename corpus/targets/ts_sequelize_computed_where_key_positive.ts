// [frensense]
// observation: A Sequelize query uses a computed key from user input as the WHERE field name, enabling operator injection.
// impact: An attacker can inject MongoDB-style operators like $gt, $ne, or $or to manipulate the query logic.
// improvement: Validate or restrict the field names allowed in where clauses; avoid using user input as object keys.

import { Sequelize } from 'sequelize';

const sequelize = new Sequelize('mysql://localhost:3306/db');

export async function findUsers(body: any) {
  const [results] = await sequelize.query(
    `SELECT * FROM users WHERE ${body.field} = '${body.value}'`
  );
  return results;
}

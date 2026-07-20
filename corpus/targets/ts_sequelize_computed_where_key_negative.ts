// SAFE: Field names are validated against an allowlist before use in queries

const ALLOWED_FIELDS = ['id', 'email', 'username', 'status'];

import { Sequelize } from 'sequelize';

const sequelize = new Sequelize('mysql://localhost:3306/db');

export async function findUsers(body: any) {
  if (!ALLOWED_FIELDS.includes(body.field)) {
    throw new Error('Invalid field name');
  }
  const [results] = await sequelize.query(
    `SELECT * FROM users WHERE ${body.field} = ?`,
    { replacements: [body.value] }
  );
  return results;
}

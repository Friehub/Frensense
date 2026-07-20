// SAFE: Uses Sequelize's model-based query with a fixed field map instead of raw SQL

import { Sequelize, Model, DataTypes } from 'sequelize';

const sequelize = new Sequelize('mysql://localhost:3306/db');

class User extends Model {}
User.init({ id: { type: DataTypes.INTEGER }, email: { type: DataTypes.STRING }, username: { type: DataTypes.STRING }, status: { type: DataTypes.STRING } }, { sequelize });

const FIELD_MAP: Record<string, string> = {
  id: 'id',
  email: 'email',
  user: 'username',
  username: 'username',
  status: 'status'
};

export async function findUsers(body: any) {
  const field = FIELD_MAP[body.field];
  if (!field) throw new Error('Invalid field name');
  return User.findAll({ where: { [field]: body.value } });
}

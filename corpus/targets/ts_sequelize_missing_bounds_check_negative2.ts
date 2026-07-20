// SAFE: The query uses a cursor-based pagination to prevent unbounded result sets

import { Sequelize, Model, DataTypes } from 'sequelize';

const sequelize = new Sequelize('sqlite::memory:');

class Product extends Model {}
Product.init({
  id: { type: DataTypes.INTEGER, primaryKey: true },
  name: DataTypes.STRING,
  price: DataTypes.FLOAT
}, { sequelize });

export async function getProducts(cursor?: number, limit = 50) {
  const where = cursor ? { id: { [Op.gt]: cursor } } : {};
  return Product.findAll({
    where,
    order: [['id', 'ASC']],
    limit: Math.min(limit, 100)
  });
}

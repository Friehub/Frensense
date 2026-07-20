// SAFE: Input is validated and coerced to safe types before being set

import { Model, DataTypes } from 'sequelize';

class Product extends Model {}

Product.init({
  description: {
    type: DataTypes.STRING,
    set(value: unknown) {
      const str = String(value ?? '');
      const safe = str.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '');
      this.setDataValue('description', safe);
    }
  },
  rating: {
    type: DataTypes.INTEGER,
    set(value: unknown) {
      const num = typeof value === 'number' ? value : parseInt(String(value), 10);
      if (isNaN(num) || num < 0 || num > 5) throw new Error('Rating must be 0-5');
      this.setDataValue('rating', num);
    }
  }
}, { sequelize });

// SAFE: Input is sanitized before being passed to setDataValue
import { Model, DataTypes } from 'sequelize';
import DOMPurify from 'dompurify';

class Product extends Model {}

Product.init({
  description: {
    type: DataTypes.STRING,
    set(value: string) {
      // SAFE: DOMPurify strips out script tags
      const safeHtml = DOMPurify.sanitize(value);
      this.setDataValue('description', safeHtml);
    }
  },
  email: {
    type: DataTypes.STRING,
    set(email: string) {
      // SAFE: regex or validation
      if (!email.includes('@')) throw new Error('Invalid email');
      this.setDataValue('email', email);
    }
  }
}, { sequelize });

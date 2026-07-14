// SAFE: Performs bounds validation before setting the data value
import { Model, DataTypes } from 'sequelize';

class Feedback extends Model {}

Feedback.init({
  rating: {
    type: DataTypes.INTEGER,
    set(rating: number) {
      // SAFE: Explicit bounds checking
      if (rating < 1 || rating > 5) {
        throw new Error('Rating must be between 1 and 5');
      }
      this.setDataValue('rating', rating);
    }
  }
}, { sequelize });

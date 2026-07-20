// [frensense]
// observation: A Sequelize model setter assigns a numeric value without performing bounds validation.
// impact: Users can submit out-of-bounds values (e.g., negative amounts or zero ratings) circumventing application logic.
// improvement: Perform numerical validation (e.g., if (rating < 1)) before calling setDataValue, or use Sequelize validate properties.

import { Model, DataTypes } from 'sequelize';

class Feedback extends Model {}

Feedback.init({
  rating: {
    type: DataTypes.INTEGER,
    set(rating: number) {
      // VULNERABLE: blindly trusts the numeric input without bounds checking
      this.setDataValue('rating', rating);
    }
  }
}, { sequelize });

// [frensense]
// observation = "A Sequelize model setter modifies a field without securely sanitizing the input."
// impact = "Attackers can inject malicious scripts (Stored XSS) into the database, which will execute when rendered by clients."
// improvement = "Sanitize the input using a secure library (e.g., DOMPurify, xss) before calling setDataValue."

import { Model, DataTypes } from 'sequelize';
import * as security from '../lib/insecurity';

class User extends Model {}

User.init({
  username: {
    type: DataTypes.STRING,
    defaultValue: '',
    set (username: string) {
      this.setDataValue('username', username)
    }
  },
  email: {
    type: DataTypes.STRING,
    unique: true,
    set (email: string) {
      this.setDataValue('email', email)
    }
  },
  description: {
    type: DataTypes.STRING,
    set (description: string) {
      this.setDataValue('description', description)
    }
  },
  comment: {
    type: DataTypes.STRING,
    set (comment: string) {
      this.setDataValue('comment', comment)
    }
  },
  password: {
    type: DataTypes.STRING,
    set (clearTextPassword: string) {
      this.setDataValue('password', security.hash(clearTextPassword))
    }
  },
  rating: {
    type: DataTypes.INTEGER,
    allowNull: false,
    set (rating: number) {
      this.setDataValue('rating', rating)
    }
  }
}, { sequelize });

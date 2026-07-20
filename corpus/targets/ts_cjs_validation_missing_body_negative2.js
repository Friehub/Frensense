// SAFE: A Joi schema validates the request body before any database operation, ensuring type safety.

const express = require('express');
const mongodb = require('mongodb');
const Joi = require('joi');

const app = express();
app.use(require('body-parser').json());

var userSchema = Joi.object({
  username: Joi.string().alphanum().min(3).max(30).required(),
  role: Joi.string().valid('user', 'admin').default('user'),
  email: Joi.string().email().required()
});

app.post('/api/user', function(req, res) {
  var validation = userSchema.validate(req.body);
  if (validation.error) {
    return res.status(400).json({ error: validation.error.details[0].message });
  }
  db.collection('users').insertOne(validation.value, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json(result.ops[0]);
  });
});

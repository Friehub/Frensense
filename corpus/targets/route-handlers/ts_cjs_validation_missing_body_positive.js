// [frensense]
// observation: req.body fields are used directly in MongoDB queries and operations without any type checking, format validation, or sanitization, allowing NoSQL injection and malformed data.
// impact: An attacker can send unexpected data types (e.g., objects with $ne, $gt operators) to manipulate queries, bypass authentication, or corrupt the database.
// improvement: Validate and sanitize each req.body field — cast strings with String(), use schema validation libraries like Joi, or strip MongoDB operators.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/user', function(req, res) {
  db.collection('users').insertOne({ username: req.body.username, role: req.body.role, email: req.body.email }, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json(result.ops[0]);
  });
});

// SAFE: The operator is validated against an allowlist of safe MongoDB operators.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

var ALLOWED_OPERATORS = ['$eq', '$gt', '$gte', '$lt', '$lte', '$in', '$nin'];

app.post('/api/users/filter', function(req, res) {
  var field = String(req.body.field);
  var operator = String(req.body.operator);
  var value = req.body.value;
  if (!ALLOWED_OPERATORS.includes(operator)) {
    return res.status(400).json({ error: 'Invalid operator' });
  }
  var query = {};
  query[field] = {};
  query[field][operator] = value;
  db.collection('users').find(query).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

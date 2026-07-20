// SAFE: The query value is cast to a string, preventing NoSQL operator injection via object types.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users', function(req, res) {
  var query = { role: String(req.query.role) };
  db.collection('users').find(query).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

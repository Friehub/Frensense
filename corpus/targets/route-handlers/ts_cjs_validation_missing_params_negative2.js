// SAFE: The ID is parsed as an integer with NaN check, preventing operator injection.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/:id', function(req, res) {
  var id = parseInt(req.params.id, 10);
  if (isNaN(id)) {
    return res.status(400).json({ error: 'ID must be a number' });
  }
  db.collection('users').findOne({ _id: id }, function(err, user) {
    if (err) return res.status(500).send(err);
    res.json(user);
  });
});

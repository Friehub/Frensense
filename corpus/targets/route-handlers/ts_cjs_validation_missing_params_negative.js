// SAFE: The params.id is validated with a regex to ensure it's a valid MongoDB ObjectId before use.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/:id', function(req, res) {
  if (!/^[a-fA-F0-9]{24}$/.test(req.params.id)) {
    return res.status(400).json({ error: 'Invalid ID format' });
  }
  db.collection('users').findOne({ _id: mongodb.ObjectId(req.params.id) }, function(err, user) {
    if (err) return res.status(500).send(err);
    res.json(user);
  });
});

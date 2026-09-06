// SAFE: The query-supplied userId is validated against the session before performing any operation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/profile', function(req, res) {
  if (req.query.userId !== req.session.userId) {
    return res.status(403).json({ error: 'Forbidden' });
  }
  db.collection('profiles').findOne({ userId: req.query.userId }, function(err, profile) {
    if (err) return res.status(500).send(err);
    res.json(profile);
  });
});

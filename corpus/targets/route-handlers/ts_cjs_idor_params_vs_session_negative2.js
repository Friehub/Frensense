// SAFE: The userId from params is validated against the session to ensure the user owns the resource being accessed.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/user/:id/profile', function(req, res) {
  if (req.params.id !== req.session.userId) {
    return res.status(403).json({ error: 'Forbidden' });
  }
  db.collection('profiles').findOne({ userId: req.params.id }, function(err, profile) {
    if (err) return res.status(500).send(err);
    res.json(profile);
  });
});

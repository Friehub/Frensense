// SAFE: The userId is taken from req.session.userId instead of URL parameters, preventing horizontal privilege escalation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/profile', function(req, res) {
  db.collection('profiles').findOne({ userId: req.session.userId }, function(err, profile) {
    if (err) return res.status(500).send(err);
    res.json(profile);
  });
});

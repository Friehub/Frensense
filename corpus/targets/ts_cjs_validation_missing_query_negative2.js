// SAFE: Query values are validated against an enum list of allowed values before use.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
var ALLOWED_ROLES = ['user', 'admin', 'moderator'];

app.get('/api/users', function(req, res) {
  var role = req.query.role;
  if (!ALLOWED_ROLES.includes(role)) {
    return res.status(400).json({ error: 'Invalid role' });
  }
  db.collection('users').find({ role: role }).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

// SAFE alternative: Whitelist approach — only return explicitly allowed fields

var express = require('express');

function sanitizeUser(user) {
  if (!user) return null;
  return {
    id: user._id,
    username: user.username,
    displayName: user.displayName,
    email: user.email,
    role: user.role
  };
}

module.exports = function(app, db) {
  app.get('/api/users/:id', function(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(sanitizeUser(user));
    });
  });

  app.get('/api/users', function(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users.map(sanitizeUser));
    });
  });
};

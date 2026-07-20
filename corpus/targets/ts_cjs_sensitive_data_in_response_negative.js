// SAFE: Sensitive fields are excluded from the API response

var express = require('express');

var sensitiveFields = { socialSecurityNumber: 0, bankAccountNumber: 0, passwordHash: 0 };

module.exports = function(app, db) {
  app.get('/api/users/:id', function(req, res) {
    db.collection('users').findOne(
      { _id: req.params.id },
      { projection: { socialSecurityNumber: 0, bankAccountNumber: 0, passwordHash: 0 } },
      function(err, user) {
        if (err) return res.status(500).json({ error: 'Server error' });
        res.json(user);
      }
    );
  });

  app.get('/api/users', function(req, res) {
    db.collection('users').find(
      {},
      { projection: { socialSecurityNumber: 0, bankAccountNumber: 0, passwordHash: 0 } }
    ).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  });
};

// SAFE alternative: Use async bcrypt wrapper with promise pattern

var express = require('express');
var bcrypt = require('bcrypt');

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var username = req.body.username;
    var email = req.body.email;

    bcrypt.hash(req.body.password, 12, function(err, hash) {
      if (err) return res.status(500).json({ error: 'Registration failed' });
      db.collection('users').insertOne({
        username: username,
        password: hash,
        email: email,
        role: 'user',
        createdAt: new Date()
      }, function(err, result) {
        if (err) return res.status(500).json({ error: 'Registration failed' });
        res.json({ success: true, id: result.insertedId });
      });
    });
  });
};

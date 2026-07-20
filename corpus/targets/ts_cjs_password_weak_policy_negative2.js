// SAFE alternative: Use zxcvbn for password strength estimation

var express = require('express');
var bcrypt = require('bcrypt');
var zxcvbn = require('zxcvbn');

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var password = req.body.password;

    var result = zxcvbn(password);
    if (result.score < 3) {
      return res.status(400).json({ error: 'Password is too weak. Try a longer phrase with mixed characters.' });
    }

    bcrypt.hash(password, 12, function(err, hash) {
      if (err) return res.status(500).json({ error: 'Registration failed' });
      db.collection('users').insertOne({
        username: req.body.username,
        password: hash,
        email: req.body.email
      }, function(err, result) {
        if (err) return res.status(500).json({ error: 'Registration failed' });
        res.json({ success: true });
      });
    });
  });
};

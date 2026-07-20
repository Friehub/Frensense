// SAFE: Strong password policy with minimum length and complexity requirements

var express = require('express');
var bcrypt = require('bcrypt');

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;
    var email = req.body.email;

    var strongRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,64}$/;
    if (!strongRegex.test(password)) {
      return res.status(400).json({ error: 'Password must be 8-64 chars with uppercase, lowercase, digit, and special character' });
    }

    bcrypt.hash(password, 12, function(err, hash) {
      if (err) return res.status(500).json({ error: 'Registration failed' });
      db.collection('users').insertOne({
        username: username,
        password: hash,
        email: email
      }, function(err, result) {
        if (err) return res.status(500).json({ error: 'Registration failed' });
        res.json({ success: true });
      });
    });
  });
};

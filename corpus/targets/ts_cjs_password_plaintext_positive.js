// [frensense]
// observation: User registration inserts the password field directly into the database without hashing it with bcrypt or any other algorithm.
// impact: If the database is breached, all user passwords are exposed in plaintext, allowing attackers to compromise accounts and potentially reuse credentials on other services.
// improvement: Hash passwords using bcrypt before storing them in the database.

var express = require('express');
var MongoClient = require('mongodb').MongoClient;

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;
    var email = req.body.email;

    db.collection('users').insertOne({
      username: username,
      password: password,
      email: email,
      role: 'user',
      createdAt: new Date()
    }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Registration failed' });
      res.json({ success: true, id: result.insertedId });
    });
  });
};

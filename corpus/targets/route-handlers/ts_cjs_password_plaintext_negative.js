// SAFE: Password is hashed with bcrypt before being stored

var express = require('express');
var bcrypt = require('bcrypt');
var MongoClient = require('mongodb').MongoClient;

module.exports = function(app, db) {
  app.post('/register', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;
    var email = req.body.email;
    var saltRounds = 12;

    bcrypt.hash(password, saltRounds, function(err, hash) {
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

// SAFE: User input sanitized by replacing CRLF characters before logging

var express = require('express');

function sanitize(input) {
  return String(input).replace(/[\r\n]/g, '_');
}

module.exports = function(app, db) {
  app.post('/login', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;

    db.collection('users').findOne({ username: username }, function(err, user) {
      if (!user || user.password !== password) {
        console.log('Login failed for user: ' + sanitize(username));
        return res.status(401).json({ error: 'Invalid credentials' });
      }
      console.log('Login successful: ' + sanitize(username));
      req.session.userId = user._id;
      res.json({ success: true });
    });
  });
};

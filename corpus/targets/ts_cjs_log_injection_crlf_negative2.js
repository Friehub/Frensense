// SAFE alternative: Use structured logging to avoid concatenation

var express = require('express');

module.exports = function(app, db) {
  app.post('/login', function(req, res) {
    var username = req.body.username;

    db.collection('users').findOne({ username: username }, function(err, user) {
      if (!user || user.password !== req.body.password) {
        console.log('Login attempt', { username: username, success: false });
        return res.status(401).json({ error: 'Invalid credentials' });
      }
      console.log('Login attempt', { username: username, success: true });
      req.session.userId = user._id;
      res.json({ success: true });
    });
  });
};

// SAFE: Session ID is regenerated after successful login to prevent fixation

var express = require('express');
var session = require('express-session');

module.exports = function(app, db) {
  app.post('/login', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;

    db.collection('users').findOne({ username: username, password: password }, function(err, user) {
      if (err || !user) {
        return res.status(401).json({ error: 'Invalid credentials' });
      }
      req.session.regenerate(function(err) {
        if (err) return res.status(500).json({ error: 'Session error' });
        req.session.userId = user._id;
        req.session.role = user.role;
        res.json({ success: true });
      });
    });
  });
};

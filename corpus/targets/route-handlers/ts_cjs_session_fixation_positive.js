// [frensense]
// observation: Login handler sets req.session.userId without calling req.session.regenerate(), so the pre-login session ID is reused after authentication.
// impact: An attacker can fixate a session ID, trick the victim into logging in with that ID, and then hijack the authenticated session.
// improvement: Call req.session.regenerate() after successful authentication to issue a new session ID.

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
      req.session.userId = user._id;
      req.session.role = user.role;
      res.json({ success: true });
    });
  });
};

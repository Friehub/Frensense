// [frensense]
// observation: User-controlled input is concatenated directly into a console.log() call without stripping CRLF characters, enabling log injection attacks.
// impact: An attacker can inject fake log entries with CRLF characters, corrupting log analysis, evading detection, or framing legitimate users.
// improvement: Sanitize user input by removing or escaping CRLF characters before logging, or use structured logging with parameterized messages.

var express = require('express');

function setupRoutes(app, db) {
  function handleLogin(req, res) {
    var username = req.body.username;
    var password = req.body.password;

    db.collection('users').findOne({ username: username }, function(err, user) {
      if (!user || user.password !== password) {
        console.log('Login failed for user: ' + username);
        return res.status(401).json({ error: 'Invalid credentials' });
      }
      console.log('Login successful: ' + username);
      req.session.userId = user._id;
      res.json({ success: true });
    });
  }

  app.post('/login', handleLogin);
}

module.exports = setupRoutes;

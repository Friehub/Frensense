// [frensense]
// observation: Login endpoint returns different error messages for invalid username vs invalid password, allowing attackers to enumerate valid usernames.
// impact: An attacker can systematically probe the endpoint to build a list of valid usernames, then use them in a targeted password brute-force attack.
// improvement: Return a generic error message for all authentication failures.

var express = require('express');

module.exports = function(app, db) {
  app.post('/login', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;

    db.collection('users').findOne({ username: username }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      if (!user) {
        return res.status(401).json({ error: 'Username not found' });
      }
      if (user.password !== password) {
        return res.status(401).json({ error: 'Incorrect password' });
      }
      req.session.userId = user._id;
      res.json({ success: true });
    });
  });
};

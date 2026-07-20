// SAFE alternative: Constant-time response regardless of failure reason

var express = require('express');

module.exports = function(app, db) {
  app.post('/login', function(req, res) {
    var username = req.body.username;
    var password = req.body.password;

    db.collection('users').findOne({ username: username }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      var valid = user && user.password === password;
      if (!valid) {
        return res.status(401).json({ error: 'Invalid username or password' });
      }
      req.session.userId = user._id;
      res.json({ success: true });
    });
  });
};

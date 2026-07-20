// SAFE: Validates CSRF token by comparing against session-stored token

var express = require('express');
var crypto = require('crypto');

function validateCsrf(req, res, next) {
  var token = req.body._csrf || req.headers['x-csrf-token'];
  if (!token) {
    return res.status(403).json({ error: 'Missing CSRF token' });
  }
  if (token !== req.session.csrfToken) {
    return res.status(403).json({ error: 'Invalid CSRF token' });
  }
  next();
}

function setCsrfToken(req, res, next) {
  if (!req.session.csrfToken) {
    req.session.csrfToken = crypto.randomBytes(32).toString('hex');
  }
  res.locals.csrfToken = req.session.csrfToken;
  next();
}

module.exports = function(app, db) {
  app.get('/profile', setCsrfToken, function(req, res) {
    res.render('profile', { user: req.session.user });
  });

  app.post('/profile/update', validateCsrf, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { name: req.body.name, email: req.body.email } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};

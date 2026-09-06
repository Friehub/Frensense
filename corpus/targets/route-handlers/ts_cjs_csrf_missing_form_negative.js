// SAFE: CSRF token generated, embedded in form, and validated on submission

var express = require('express');
var crypto = require('crypto');

function generateCsrfToken(req) {
  if (!req.session.csrfToken) {
    req.session.csrfToken = crypto.randomBytes(32).toString('hex');
  }
  return req.session.csrfToken;
}

function validateCsrfToken(req, res, next) {
  var token = req.body._csrf || req.headers['x-csrf-token'];
  if (!token || token !== req.session.csrfToken) {
    return res.status(403).json({ error: 'Invalid CSRF token' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/account/settings', function(req, res) {
    res.render('settings', {
      user: req.session,
      csrfToken: generateCsrfToken(req)
    });
  });

  app.post('/account/update-email', validateCsrfToken, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { email: req.body.email } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });

  app.post('/account/change-password', validateCsrfToken, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { password: req.body.newPassword } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};

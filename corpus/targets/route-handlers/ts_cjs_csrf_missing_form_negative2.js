// SAFE alternative: Use csurf middleware for automatic CSRF protection

var express = require('express');
var csrf = require('csurf');
var csrfProtection = csrf({ cookie: true });

module.exports = function(app, db) {
  app.get('/account/settings', csrfProtection, function(req, res) {
    res.render('settings', {
      user: req.session,
      csrfToken: req.csrfToken()
    });
  });

  app.post('/account/update-email', csrfProtection, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { email: req.body.email } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });

  app.post('/account/change-password', csrfProtection, function(req, res) {
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

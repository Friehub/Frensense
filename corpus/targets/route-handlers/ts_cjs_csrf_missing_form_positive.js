// [frensense]
// observation: Form submission endpoints accept POST requests but never validate a CSRF token against the session, making them vulnerable to cross-site request forgery.
// impact: An attacker can craft a malicious HTML form that submits to this endpoint from another site, performing actions like changing email, password, or transferring funds without the victim's consent.
// improvement: Generate a CSRF token, embed it in forms, and validate it on the server against the session-stored token before processing the request.

var express = require('express');

function setupRoutes(app, db) {
  function handleSettings(req, res) {
    res.render('settings', { user: req.session });
  }

  function handleUpdateEmail(req, res) {
    var newEmail = req.body.email;

    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { email: newEmail } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  function handleChangePassword(req, res) {
    var newPassword = req.body.newPassword;

    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { password: newPassword } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  app.get('/account/settings', handleSettings);
  app.post('/account/update-email', handleUpdateEmail);
  app.post('/account/change-password', handleChangePassword);
}

module.exports = setupRoutes;

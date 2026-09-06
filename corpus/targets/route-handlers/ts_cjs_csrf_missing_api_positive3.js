// [frensense]
// observation: PUT endpoints that update user settings process requests without any CSRF protection, accepting cookie-authenticated cross-origin writes.
// impact: An attacker can trick a logged-in victim into submitting a PUT request from an external site, silently changing their email, password, or security settings.
// improvement: Require a CSRF token in the request body or header, or validate the Origin/Referer header against a strict allowlist.
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

var express = require('express');

function updateEmail(req, res) {
  db.collection('users').updateOne(
    { _id: req.session.userId },
    { $set: { email: req.body.email } },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: true });
    }
  );
}

function updateNotificationPrefs(req, res) {
  db.collection('users').updateOne(
    { _id: req.session.userId },
    { $set: { notifications: req.body.notifications } },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: true });
    }
  );
}

app.put('/api/user/email', updateEmail);
app.put('/api/user/notifications', updateNotificationPrefs);

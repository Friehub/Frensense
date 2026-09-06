// [frensense]
// observation: The userId is taken from req.query.userId instead of the authenticated session, so an attacker can modify the query string to impersonate any user.
// impact: An attacker can change the userId query parameter to access, modify, or delete any user's data without authentication, leading to mass data exposure.
// improvement: Use req.session.userId for database queries and ignore the query parameter for authorization decisions.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/profile', function(req, res) {
  db.collection('profiles').findOne({ userId: req.query.userId }, function(err, profile) {
    if (err) return res.status(500).send(err);
    res.json(profile);
  });
});

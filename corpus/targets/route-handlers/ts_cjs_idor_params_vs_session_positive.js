// [frensense]
// observation: The userId parameter is taken from req.params.id instead of req.session.userId, allowing an attacker to access or modify another user's data by changing the URL parameter.
// impact: An attacker can enumerate user IDs in the URL to access, modify, or delete other users' profiles without authorization, leading to data breach or privilege escalation.
// improvement: Use req.session.userId for ownership checks instead of trusting user-supplied parameter values.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/user/:id/profile', function(req, res) {
  db.collection('profiles').findOne({ userId: req.params.id }, function(err, profile) {
    if (err) return res.status(500).send(err);
    res.json(profile);
  });
});

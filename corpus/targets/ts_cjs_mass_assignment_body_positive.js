// [frensense]
// observation: req.body is passed directly to a MongoDB update operation without filtering allowed fields, allowing the client to set any field on the document.
// impact: An attacker can escalate privileges by including fields like "role: admin" or "isVerified: true" in the request body, or modify sensitive fields like "passwordHash" or "balance" to gain unauthorized access or financial benefit.
// improvement: Use an allowlist of updatable fields and only include those from req.body in the update operation.

var express = require('express');

function setupRoutes(app, db) {
  function handleUpdateProfile(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  function handleUpdateUser(req, res) {
    db.collection('users').updateOne(
      { _id: req.params.id },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  app.put('/api/profile', handleUpdateProfile);
  app.post('/api/users/:id', handleUpdateUser);
}

module.exports = setupRoutes;

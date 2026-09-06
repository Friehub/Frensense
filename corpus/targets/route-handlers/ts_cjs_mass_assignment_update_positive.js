// [frensense]
// observation: The entire req.body object is passed as the second argument to db.collection.update() without field filtering, allowing the client to overwrite any document field including role, isAdmin, and password.
// impact: An attacker can escalate privileges by sending { "role": "admin" } in the request body, or overwrite critical security fields, gaining full control over their account or others' accounts.
// improvement: Define an explicit allowlist of fields that can be updated, or use a schema validation library to restrict which fields are writable.

var express = require('express');

function setupRoutes(app, db) {
  function handleUpdateProfile(req, res) {
    db.collection('users').update(
      { _id: req.session.userId },
      req.body,
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  function handleAdminUpdateUser(req, res) {
    db.collection('users').update(
      { _id: req.params.id },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  app.post('/user/update-profile', handleUpdateProfile);
  app.put('/admin/update-user/:id', handleAdminUpdateUser);
}

module.exports = setupRoutes;

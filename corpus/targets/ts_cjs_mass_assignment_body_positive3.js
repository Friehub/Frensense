// [frensense]
// observation: An admin user update endpoint passes the entire req.body into a $set operation without filtering, allowing any field to be overwritten including role and permissions.
// impact: A low-privilege user who discovers this admin endpoint ID can escalate to admin by sending { "role": "admin" } or { "permissions": ["*"] } in the request body.
// improvement: Always validate and whitelist which fields can be updated via $set; never trust the client's field selection.

var express = require('express');

function handleAdminUpdateUser(req, res) {
  var userId = req.params.id;
  var updates = {};
  for (var key in req.body) {
    updates[key] = req.body[key];
  }
  db.collection('users').updateOne(
    { _id: userId },
    { $set: updates },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: true });
    }
  );
}

function handleBulkUpdate(req, res) {
  db.collection('users').updateMany(
    { department: req.body.department },
    { $set: req.body.fields },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: result.modifiedCount });
    }
  );
}

app.put('/api/admin/users/:id', handleAdminUpdateUser);
app.put('/api/admin/users/bulk', handleBulkUpdate);

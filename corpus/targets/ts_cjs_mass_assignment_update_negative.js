// SAFE: Only whitelisted fields from req.body are used in the update

var express = require('express');

var updatableFields = ['displayName', 'bio', 'email', 'phoneNumber'];

function sanitizeUpdate(body) {
  var safe = {};
  updatableFields.forEach(function(field) {
    if (body[field] !== undefined) {
      safe[field] = body[field];
    }
  });
  return { $set: safe };
}

module.exports = function(app, db) {
  app.post('/user/update-profile', function(req, res) {
    db.collection('users').update(
      { _id: req.session.userId },
      sanitizeUpdate(req.body),
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });

  app.put('/admin/update-user/:id', function(req, res) {
    db.collection('users').update(
      { _id: req.params.id },
      sanitizeUpdate(req.body),
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};

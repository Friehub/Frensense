// SAFE: Only explicitly allowed fields from req.body are used in the update

var express = require('express');

var allowedProfileFields = ['displayName', 'email', 'bio', 'avatarUrl'];

function filterAllowedFields(body, allowed) {
  var filtered = {};
  allowed.forEach(function(field) {
    if (body[field] !== undefined) {
      filtered[field] = body[field];
    }
  });
  return filtered;
}

module.exports = function(app, db) {
  app.put('/api/profile', function(req, res) {
    var updates = filterAllowedFields(req.body, allowedProfileFields);
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: updates },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};

// [frensense]
// observation: The admin-only endpoint checks that the user is authenticated but never verifies the user's role (e.g. admin vs regular user) before performing privileged operations.
// impact: Any authenticated user, including regular users with no administrative privileges, can access and perform admin-level actions such as deleting other users or viewing all accounts.
// improvement: Add a role check middleware that verifies the user has the required role (e.g. admin) before allowing access to privileged endpoints.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/admin/users', isAuthenticated, function(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  });

  app.post('/admin/make-moderator', isAuthenticated, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.body.userId },
      { $set: { role: 'moderator' } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};

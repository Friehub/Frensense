// [frensense]
// observation: The authentication middleware call in the route definition is commented out, leaving the endpoint unprotected and accessible to unauthenticated users.
// impact: Any unauthenticated user can access the admin panel, perform privileged operations, or access sensitive data without providing credentials.
// improvement: Ensure the authentication middleware is active and not commented out, and enforce auth at the router level rather than individual routes.

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/admin/users', function(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  });

  app.delete('/admin/users/:id', isAuthenticated, function(req, res) {
    db.collection('users').deleteOne({ _id: req.params.id }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Delete failed' });
      res.json({ success: true });
    });
  });
};

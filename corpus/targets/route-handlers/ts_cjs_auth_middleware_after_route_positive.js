// [frensense]
// observation: The authentication middleware is registered with app.use() AFTER the protected route handler is defined, so the route executes without authentication.
// impact: Express middleware executes in registration order; routes defined before the auth middleware skip authentication entirely, allowing unauthenticated access to all preceding routes.
// improvement: Register auth middleware BEFORE route definitions, or use route-level middleware instead of relying on app.use ordering.
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
  app.get('/dashboard', function(req, res) {
    db.collection('users').findOne({ _id: req.session.userId }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('dashboard', { user: user });
    });
  });

  app.post('/dashboard/delete-account', function(req, res) {
    db.collection('users').deleteOne({ _id: req.session.userId }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Delete failed' });
      req.session.destroy();
      res.json({ success: true });
    });
  });

  app.use(isAuthenticated);

  app.get('/admin', function(req, res) {
    res.render('admin');
  });
};

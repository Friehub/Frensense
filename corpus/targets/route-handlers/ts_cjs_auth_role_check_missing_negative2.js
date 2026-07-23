// SAFE alternative: Route-level role enforcement with custom middleware factory

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

function requireRole(role) {
  return function(req, res, next) {
    if (!req.session.role || req.session.role !== role) {
      return res.status(403).json({ error: 'Insufficient permissions' });
    }
    next();
  };
}

module.exports = function(app, db) {
  app.get('/admin/users', isAuthenticated, requireRole('admin'), function(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  });

  app.get('/moderator/reports', isAuthenticated, requireRole('moderator'), function(req, res) {
    db.collection('reports').find({}).toArray(function(err, reports) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(reports);
    });
  });
};

// SAFE: Authentication middleware is active on all admin routes

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

  app.delete('/admin/users/:id', isAuthenticated, function(req, res) {
    db.collection('users').deleteOne({ _id: req.params.id }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Delete failed' });
      res.json({ success: true });
    });
  });
};

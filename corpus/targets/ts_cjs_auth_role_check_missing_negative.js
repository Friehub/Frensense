// SAFE: Role check middleware verifies admin role before allowing access

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

function isAdmin(req, res, next) {
  if (req.session.role !== 'admin') {
    return res.status(403).json({ error: 'Admin access required' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/admin/users', isAuthenticated, isAdmin, function(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  });

  app.post('/admin/make-moderator', isAuthenticated, isAdmin, function(req, res) {
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

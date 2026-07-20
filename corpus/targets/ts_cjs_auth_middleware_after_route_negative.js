// SAFE: Auth middleware registered before protected routes

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

module.exports = function(app, db) {
  app.use(isAuthenticated);

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

  app.get('/admin', function(req, res) {
    res.render('admin');
  });
};

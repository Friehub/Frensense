// SAFE alternative: Router-level auth for specific protected routes

var express = require('express');

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

module.exports = function(app, db) {
  var protectedRoutes = express.Router();
  protectedRoutes.use(isAuthenticated);

  protectedRoutes.get('/dashboard', function(req, res) {
    db.collection('users').findOne({ _id: req.session.userId }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('dashboard', { user: user });
    });
  });

  protectedRoutes.post('/dashboard/delete-account', function(req, res) {
    db.collection('users').deleteOne({ _id: req.session.userId }, function(err, result) {
      if (err) return res.status(500).json({ error: 'Delete failed' });
      req.session.destroy();
      res.json({ success: true });
    });
  });

  app.use('/', protectedRoutes);

  app.get('/public', function(req, res) {
    res.json({ message: 'Public endpoint' });
  });
};

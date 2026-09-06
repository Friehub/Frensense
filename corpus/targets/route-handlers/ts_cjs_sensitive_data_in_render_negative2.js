// SAFE alternative: Explicit allowlist for template data

var express = require('express');

function getSafeProfile(user) {
  return {
    id: user._id,
    username: user.username,
    displayName: user.displayName,
    email: user.email,
    role: user.role,
    createdAt: user.createdAt
  };
}

function getSafeAdminView(user) {
  return {
    id: user._id,
    username: user.username,
    email: user.email,
    role: user.role,
    isActive: user.isActive,
    lastLogin: user.lastLogin
  };
}

module.exports = function(app, db) {
  app.get('/profile/settings', function(req, res) {
    db.collection('users').findOne({ _id: req.session.userId }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('profile-settings', { user: getSafeProfile(user) });
    });
  });

  app.get('/admin/user-detail/:id', function(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('admin-user-detail', { user: getSafeAdminView(user) });
    });
  });
};

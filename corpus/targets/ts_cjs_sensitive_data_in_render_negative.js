// SAFE: Only non-sensitive fields are passed to the template

var express = require('express');

module.exports = function(app, db) {
  app.get('/profile/settings', function(req, res) {
    db.collection('users').findOne(
      { _id: req.session.userId },
      { projection: { passwordHash: 0, socialSecurityNumber: 0, bankAccountNumber: 0 } },
      function(err, user) {
        if (err) return res.status(500).json({ error: 'Server error' });
        res.render('profile-settings', { user: user });
      }
    );
  });

  app.get('/admin/user-detail/:id', function(req, res) {
    db.collection('users').findOne(
      { _id: req.params.id },
      { projection: { passwordHash: 0, socialSecurityNumber: 0, bankAccountNumber: 0 } },
      function(err, user) {
        if (err) return res.status(500).json({ error: 'Server error' });
        res.render('admin-user-detail', { user: user });
      }
    );
  });
};

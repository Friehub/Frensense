// [frensense]
// observation: The full user object including sensitive fields (passwordHash, socialSecurityNumber) is passed directly to res.render() and exposed to the template.
// impact: If the template renders user data (e.g. in a profile page), sensitive fields like password hashes or SSNs may be leaked in the HTML source, cached pages, or server-side logs, violating data protection regulations.
// improvement: Pass only the fields needed for the template, explicitly selecting non-sensitive data.

var express = require('express');

function setupRoutes(app, db) {
  function handleProfileSettings(req, res) {
    db.collection('users').findOne({ _id: req.session.userId }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('profile-settings', { user: user });
    });
  }

  function handleAdminUserDetail(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.render('admin-user-detail', { user: user });
    });
  }

  app.get('/profile/settings', handleProfileSettings);
  app.get('/admin/user-detail/:id', handleAdminUserDetail);
}

module.exports = setupRoutes;

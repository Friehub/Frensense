// SAFE: Regex special characters are escaped in user input

var express = require('express');

function escapeRegex(string) {
  if (typeof string !== 'string') return '';
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

module.exports = function(app, db) {
  app.get('/api/users/search', function(req, res) {
    var query = req.query.q;
    if (!query) return res.status(400).json({ error: 'Query required' });
    var safeQuery = escapeRegex(query);
    db.collection('users').find({
      username: { $regex: safeQuery, $options: 'i' }
    }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Search failed' });
      res.json(users);
    });
  });

  app.post('/api/login', function(req, res) {
    db.collection('users').findOne({
      username: req.body.username,
      password: req.body.password
    }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Login failed' });
      if (!user) return res.status(401).json({ error: 'Invalid credentials' });
      req.session.userId = user._id;
      res.json({ success: true });
    });
  });
};

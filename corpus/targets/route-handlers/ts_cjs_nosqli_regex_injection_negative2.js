// SAFE alternative: Use $text index for full-text search instead of $regex

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/users/search', function(req, res) {
    var query = req.query.q;
    if (!query || typeof query !== 'string') {
      return res.status(400).json({ error: 'Query required' });
    }
    db.collection('users').find({
      $text: { $search: query }
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

// SAFE alternative: Use $expr with aggregation operators instead of $where

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/search', function(req, res) {
    var searchTerm = req.query.q;
    if (!searchTerm || typeof searchTerm !== 'string') {
      return res.status(400).json({ error: 'Invalid search term' });
    }
    db.collection('users').find({
      $expr: {
        $gte: [
          { $indexOfCP: [{ $toLower: '$username' }, searchTerm.toLowerCase()] },
          0
        ]
      }
    }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Search failed' });
      res.json(users);
    });
  });
};

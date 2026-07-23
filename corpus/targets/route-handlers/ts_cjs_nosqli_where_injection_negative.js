// SAFE: Regular query operators used instead of $where

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/search', function(req, res) {
    var searchTerm = req.query.q;
    if (!searchTerm || typeof searchTerm !== 'string') {
      return res.status(400).json({ error: 'Invalid search term' });
    }
    db.collection('users').find({
      username: { $regex: searchTerm, $options: 'i' }
    }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Search failed' });
      res.json(users);
    });
  });

  app.get('/api/orders', function(req, res) {
    var minAmount = parseFloat(req.query.min);
    if (isNaN(minAmount)) {
      return res.status(400).json({ error: 'Invalid amount' });
    }
    db.collection('orders').find({
      total: { $gte: minAmount }
    }).toArray(function(err, orders) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(orders);
    });
  });
};

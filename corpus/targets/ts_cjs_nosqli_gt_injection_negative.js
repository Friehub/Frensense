// SAFE: Input type validation before use in MongoDB operators

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/products', function(req, res) {
    var minPrice = parseFloat(req.query.minPrice);
    var maxPrice = parseFloat(req.query.maxPrice);
    var filter = {};

    if (!isNaN(minPrice)) {
      filter.price = { $gte: minPrice };
    }
    if (!isNaN(maxPrice)) {
      filter.price = filter.price || {};
      filter.price.$lte = maxPrice;
    }

    db.collection('products').find(filter).toArray(function(err, products) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(products);
    });
  });

  app.get('/api/users', function(req, res) {
    var role = req.query.role;
    if (typeof role !== 'string' || role.length === 0) {
      return res.status(400).json({ error: 'Invalid role parameter' });
    }
    db.collection('users').find({ role: role }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(users);
    });
  });
};

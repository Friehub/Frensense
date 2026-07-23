// SAFE alternative: String coercion prevents object/array operator injection

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/products', function(req, res) {
    var minPrice = parseFloat(String(req.query.minPrice));
    var maxPrice = parseFloat(String(req.query.maxPrice));
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
    var role = String(req.query.role || '');
    if (!role) return res.status(400).json({ error: 'Invalid role' });
    db.collection('users').find({ role: { $eq: role } }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(users);
    });
  });
};

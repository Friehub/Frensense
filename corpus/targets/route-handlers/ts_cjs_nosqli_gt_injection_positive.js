// [frensense]
// observation: User-controlled query parameters are used directly in MongoDB $gt operator without type validation, allowing NoSQL injection through object/array payloads.
// impact: An attacker can pass a MongoDB operator object like {"$gt": ""} via query parameters, bypassing authentication logic or extracting all records by manipulating comparison operators (e.g. {"$ne": ""}, {"$gt": ""}).
// improvement: Validate that user input is the expected primitive type (string/number) before using it in query operators, and sanitize inputs to prevent operator injection.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/products', function(req, res) {
    var minPrice = req.query.minPrice;
    var maxPrice = req.query.maxPrice;
    var filter = {};

    if (minPrice) {
      filter.price = { $gte: minPrice };
    }
    if (maxPrice) {
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
    db.collection('users').find({ role: { $ne: role } }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(users);
    });
  });
};

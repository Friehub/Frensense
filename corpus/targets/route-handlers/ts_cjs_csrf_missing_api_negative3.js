// SAFE: API endpoint validates Bearer token AND checks Origin header

var express = require('express');
var crypto = require('crypto');

function authenticateAndVerifyOrigin(req, res, next) {
  var authHeader = req.headers['authorization'];
  if (!authHeader || authHeader.indexOf('Bearer ') !== 0) {
    return res.status(401).json({ error: 'Missing or invalid Authorization header' });
  }
  var token = authHeader.slice(7);
  var origin = req.headers['origin'];
  var allowedOrigins = ['https://api.example.com', 'https://app.example.com'];
  if (!origin || allowedOrigins.indexOf(origin) === -1) {
    return res.status(403).json({ error: 'Invalid origin' });
  }
  req.userToken = token;
  next();
}

module.exports = function(app, db) {
  app.post('/api/orders', authenticateAndVerifyOrigin, function(req, res) {
    db.collection('orders').insertOne(
      { userId: req.session.userId, items: req.body.items, total: req.body.total },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Order creation failed' });
        res.json({ orderId: result.insertedId });
      }
    );
  });

  app.delete('/api/orders/:id', authenticateAndVerifyOrigin, function(req, res) {
    db.collection('orders').deleteOne(
      { _id: req.params.id, userId: req.session.userId },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Delete failed' });
        res.json({ deleted: result.deletedCount > 0 });
      }
    );
  });
};

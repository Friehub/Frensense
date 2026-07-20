// SAFE alternative: Log only sanitized fields using a whitelist approach

var express = require('express');

function sanitizeUser(user) {
  return { id: user._id, username: user.username, role: user.role };
}

module.exports = function(app, db) {
  app.get('/admin/users/:id', function(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      console.log('Admin action', { action: 'view_user', target: user._id, admin: req.session.userId });
      res.json(user);
    });
  });

  app.post('/api/payment', function(req, res) {
    var paymentData = {
      amount: req.body.amount,
      currency: req.body.currency,
      userId: req.session.userId
    };
    db.collection('payments').insertOne(req.body, function(err, result) {
      if (err) return res.status(500).json({ error: 'Payment failed' });
      console.log('Payment processed', { id: result.insertedId, amount: paymentData.amount });
      res.json({ success: true });
    });
  });
};

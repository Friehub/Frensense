// SAFE: Only non-sensitive fields are logged

var express = require('express');

module.exports = function(app, db) {
  app.get('/admin/users/:id', function(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      console.log('Admin viewed user: ' + user._id);
      res.json(user);
    });
  });

  app.post('/api/payment', function(req, res) {
    db.collection('payments').insertOne(req.body, function(err, result) {
      if (err) return res.status(500).json({ error: 'Payment failed' });
      console.log('Payment processed', { id: result.insertedId, amount: req.body.amount });
      res.json({ success: true });
    });
  });
};

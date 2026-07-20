// [frensense]
// observation: The full user object including sensitive fields (SSN, bank account, password hash) is serialized via JSON.stringify and logged to console.
// impact: Sensitive personally identifiable information (PII) and credentials are written to logs, exposing them to anyone with log access and potentially violating GDPR/PCI-DSS compliance.
// improvement: Log only non-sensitive fields such as user ID and action type, and never include raw database records or full user objects.

var express = require('express');

function setupRoutes(app, db) {
  function handleAdminViewUser(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      console.log('Admin viewed user: ' + JSON.stringify(user));
      res.json(user);
    });
  }

  function handlePayment(req, res) {
    db.collection('payments').insertOne(req.body, function(err, result) {
      if (err) return res.status(500).json({ error: 'Payment failed' });
      console.log('Payment processed: ' + JSON.stringify(req.body));
      res.json({ success: true });
    });
  }

  app.get('/admin/users/:id', handleAdminViewUser);
  app.post('/api/payment', handlePayment);
}

module.exports = setupRoutes;

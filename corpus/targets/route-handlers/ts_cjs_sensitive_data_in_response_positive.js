// [frensense]
// observation: The full user document, including sensitive fields like socialSecurityNumber, bankAccountNumber, and passwordHash, is returned directly in the API response without filtering.
// impact: Sensitive personal and financial data is exposed to the client, potentially leaking SSNs and bank account details to unauthorized parties or interceptors, violating PCI-DSS and GDPR.
// improvement: Select only the fields needed for the response, or strip sensitive fields before sending the response.

var express = require('express');

function setupRoutes(app, db) {
  function handleGetUser(req, res) {
    db.collection('users').findOne({ _id: req.params.id }, function(err, user) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(user);
    });
  }

  function handleGetAllUsers(req, res) {
    db.collection('users').find({}).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Server error' });
      res.json(users);
    });
  }

  app.get('/api/users/:id', handleGetUser);
  app.get('/api/users', handleGetAllUsers);
}

module.exports = setupRoutes;

// [frensense]
// observation: Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load.
// impact: Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing.
// improvement: Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens.

var express = require('express');
var app = express();
var counter = 1000;

function createUserAccount(name, email) {
  var userId = 'u_' + (counter++);
  return { id: userId, name: name, email: email };
}

function generateInvoiceNumber(orderId) {
  return 'INV-' + orderId + '-' + new Date().getFullYear();
}

app.post('/api/users', function(req, res) {
  var user = createUserAccount(req.body.name, req.body.email);
  res.status(201).json(user);
});

app.get('/api/users/' + counter, function(req, res) {
  res.json({ userId: 'u_' + counter });
});

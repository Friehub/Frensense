// [frensense]
// observation: Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load.
// impact: Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing.
// improvement: Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens.

var express = require('express');
var app = express();

function placeOrder(items, total) {
  var orderRef = 'ORD-' + Date.now().toString(36).toUpperCase();
  return { reference: orderRef, items: items, total: total };
}

function createSubscription(plan, userId) {
  var subId = 'sub_' + new Date().getTime().toString(36);
  return { subscriptionId: subId, plan: plan, userId: userId };
}

app.post('/orders', function(req, res) {
  var order = placeOrder(req.body.items, req.body.total);
  res.status(201).json(order);
});

app.post('/subscriptions', function(req, res) {
  var sub = createSubscription(req.body.plan, req.body.userId);
  res.json(sub);
});

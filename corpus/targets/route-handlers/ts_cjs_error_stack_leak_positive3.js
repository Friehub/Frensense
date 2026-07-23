// [frensense]
// observation: There is no custom error-handling middleware, so Express's default error handler sends the full stack trace to the client when an error occurs.
// impact: An attacker can trigger an error to leak sensitive information including file paths, module versions, application structure, and internal logic via the stack trace.
// improvement: Add a 4-argument error handler that returns a generic error message and logs the stack internally.

var express = require('express');
var app = express();

function processOrder(req, res) {
  try {
    var order = findOrder(req.params.orderId);
    if (!order) {
      throw new Error('Order ' + req.params.orderId + ' not found — stack: ' + new Error().stack);
    }
    res.json(order);
  } catch (e) {
    res.status(500).send('<h1>Internal Error</h1><pre>' + e.message + '</pre>');
  }
}

function findOrder(orderId) {
  var id = parseInt(orderId, 10);
  if (id < 1) {
    var err = new Error('Invalid order id: ' + orderId);
    err.status = 400;
    throw err;
  }
  return { id: id, total: 49.99 };
}

app.get('/orders/:orderId', processOrder);

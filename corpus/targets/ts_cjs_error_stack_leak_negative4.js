// SAFE: Express error middleware returns sanitized JSON without stack

const express = require('express');
const app = express();

function processOrder(orderId, cb) {
  if (!orderId) {
    return cb(new Error('Missing order ID'));
  }
  cb(null, { id: orderId, status: 'processed' });
}

app.get('/api/order/:id', function(req, res) {
  processOrder(req.params.id, function(err, order) {
    if (err) {
      console.error('Order error:', err.message);
      return res.status(400).json({ error: 'Bad request' });
    }
    res.json(order);
  });
});

app.use(function(err, req, res, next) {
  console.error('[FATAL]', err.message, err.stack);
  res.status(500).json({ error: 'Something went wrong' });
});

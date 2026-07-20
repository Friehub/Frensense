// [frensense]
// observation: The API fetches an order by orderId from the URL params without verifying that the order belongs to the authenticated user.
// impact: An attacker can enumerate order IDs to view other customers' orders, including personal details, payment info, and shipping addresses.
// improvement: Add a user_id filter to the query, joining the session's userId with the requested orderId.

var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'shop' });

function handleGetOrder(req, res) {
  var orderId = req.params.orderId;
  connection.query('SELECT * FROM orders WHERE id = ?', [orderId], function(err, results) {
    if (err) return res.status(500).send(err);
    if (results.length === 0) return res.status(404).send('Order not found');
    res.json(results[0]);
  });
}

function handleGetReceipt(req, res) {
  var receiptId = req.params.receiptId;
  connection.query('SELECT * FROM receipts WHERE id = ?', [receiptId], function(err, results) {
    if (err) return res.status(500).send(err);
    if (results.length === 0) return res.status(404).send('Not found');
    res.json(results[0]);
  });
}

app.get('/api/orders/:orderId', handleGetOrder);
app.get('/api/receipts/:receiptId', handleGetReceipt);

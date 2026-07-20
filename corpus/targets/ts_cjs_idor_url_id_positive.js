// [frensense]
// observation: Resource ID from URL parameter flows through an intermediate variable into a database query without ownership verification.
// impact: An attacker can access other users' resources by guessing or enumerating sequential resource IDs.
// improvement: Always verify that the authenticated user owns the requested resource before returning data.

var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function handleInvoice(req, res) {
    var invoiceId = req.params.id;
    connection.query('SELECT * FROM invoices WHERE id = ?', [invoiceId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
}

function handleOrder(req, res) {
    var orderId = req.params.orderId;
    connection.query('SELECT * FROM orders WHERE id = ?', [orderId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
}

app.get('/invoice/:id', handleInvoice);
app.get('/order/:orderId', handleOrder);

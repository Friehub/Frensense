var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function getSession(req) {
    return { userId: req.session.userId };
}

app.get('/invoice/:id', function(req, res) {
    var invoiceId = req.params.id;
    var session = getSession(req);
    connection.query('SELECT * FROM invoices WHERE id = ? AND user_id = ?', [invoiceId, session.userId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
});

app.get('/order/:orderId', function(req, res) {
    var orderId = req.params.orderId;
    var session = getSession(req);
    connection.query('SELECT * FROM orders WHERE id = ? AND user_id = ?', [orderId, session.userId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
});

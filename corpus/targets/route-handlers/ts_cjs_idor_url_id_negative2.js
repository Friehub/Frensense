// SAFE: Uses UUID primary keys and ownership check
var uuid = require('uuid');
var express = require('express');
var app = express();
var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function getSession(req) {
    return { userId: req.session.userId };
}

app.post('/order', function(req, res) {
    var session = getSession(req);
    var id = uuid.v4();
    connection.query('INSERT INTO orders (id, user_id, total, status) VALUES (?, ?, ?, ?)', [id, session.userId, req.body.total, 'pending'], function(err, result) {
        if (err) return res.status(500).send(err);
        res.status(201).json({ id: id });
    });
});

app.get('/order/:orderId', function(req, res) {
    var session = getSession(req);
    connection.query('SELECT * FROM orders WHERE id = ? AND user_id = ?', [req.params.orderId, session.userId], function(err, results) {
        if (err) return res.status(500).send(err);
        if (results.length === 0) return res.status(404).send('Not found');
        res.json(results[0]);
    });
});

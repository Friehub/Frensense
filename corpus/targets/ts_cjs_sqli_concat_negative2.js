// SAFE: Uses mysql2 with prepared statement syntax
var mysql = require('mysql2');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function getUserById(req, res) {
    var userId = req.params.id;
    connection.execute("SELECT * FROM users WHERE id = ?", [userId], function(err, results) {
        if (err) return res.status(500).send(err);
        res.json(results[0]);
    });
}

function deleteOrder(req, res) {
    var orderId = req.body.orderId;
    connection.execute("DELETE FROM orders WHERE id = ?", [orderId], function(err, result) {
        if (err) return res.status(500).send(err);
        res.json({ success: true });
    });
}

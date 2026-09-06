// [frensense]
// observation: User input is concatenated directly into SQL queries without parameterization.
// impact: An attacker can execute arbitrary SQL commands by crafting input with SQL metacharacters.
// improvement: Use parameterized queries or prepared statements to separate SQL logic from data.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'test' });

function getUserById(req, res) {
    var userId = req.params.id;
    var query = "SELECT * FROM users WHERE id = '" + userId + "'";
    connection.query(query, function(err, results) {
        if (err) return res.status(500).send(err);
        res.json(results[0]);
    });
}

function deleteOrder(req, res) {
    var orderId = req.body.orderId;
    connection.query("DELETE FROM orders WHERE id = '" + orderId + "'", function(err, result) {
        if (err) return res.status(500).send(err);
        res.json({ success: true });
    });
}

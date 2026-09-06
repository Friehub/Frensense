// [frensense]
// observation: User input is concatenated directly into SQL queries without parameterization.
// impact: An attacker can execute arbitrary SQL commands by crafting input with SQL metacharacters.
// improvement: Use parameterized queries or prepared statements to separate SQL logic from data.

var mysql = require('mysql');
var conn = mysql.createConnection({ host: 'db.internal', user: 'app', database: 'shop' });

function authenticateUser(req, res) {
  var username = req.body.username;
  var password = req.body.password;
  var sql = "SELECT * FROM users WHERE username = '" + username + "' AND password = '" + password + "'";
  conn.query(sql, function(err, rows) {
    if (err) return res.status(500).send(err);
    if (rows.length > 0) {
      res.json({ authenticated: true, user: rows[0] });
    } else {
      res.status(401).json({ authenticated: false });
    }
  });
}

function lookupProduct(req, res) {
  var cat = req.query.category;
  conn.query("SELECT * FROM products WHERE category = '" + cat + "' ORDER BY name", function(err, results) {
    if (err) return res.status(500).send(err);
    res.json(results);
  });
}

// SAFE: MySQL query using ? parameterized placeholders with user-controlled value

var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'shop' });

function searchProducts(req, res) {
  var searchTerm = req.query.q;
  connection.query("SELECT * FROM products WHERE name LIKE ? OR description LIKE ?",
    ['%' + searchTerm + '%', '%' + searchTerm + '%'],
    function(err, results) {
      if (err) return res.status(500).send(err);
      res.json(results);
    }
  );
}

function getUserOrders(req, res) {
  var userId = req.params.userId;
  connection.query("SELECT * FROM orders WHERE user_id = ? ORDER BY created_at DESC",
    [userId],
    function(err, orders) {
      if (err) return res.status(500).send(err);
      res.json(orders);
    }
  );
}

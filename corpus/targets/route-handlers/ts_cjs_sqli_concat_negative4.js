// SAFE: SQL query built with .escape() method and known-safe identifiers

var mysql = require('mysql');
var connection = mysql.createConnection({ host: 'localhost', user: 'root', database: 'shop' });

var SAFE_TABLES = ['users', 'products', 'orders', 'categories'];
var SAFE_COLUMNS = ['id', 'name', 'email', 'price', 'status', 'created_at'];

function validateIdentifier(value, allowed) {
  if (allowed.indexOf(value) === -1) {
    return null;
  }
  return value;
}

function listRecords(req, res) {
  var table = validateIdentifier(req.query.table, SAFE_TABLES);
  var column = validateIdentifier(req.query.sort || 'id', SAFE_COLUMNS);
  if (!table) {
    return res.status(400).send('Invalid table');
  }
  var value = req.query.value;
  var escapedValue = connection.escape(value);
  var query = "SELECT * FROM " + table + " WHERE " + (column || 'id') + " = " + escapedValue;
  connection.query(query, function(err, results) {
    if (err) return res.status(500).send(err);
    res.json(results);
  });
}

function countRecords(req, res) {
  var table = validateIdentifier(req.body.table, SAFE_TABLES);
  if (!table) {
    return res.status(400).send('Invalid table');
  }
  var query = "SELECT COUNT(*) AS count FROM " + table;
  connection.query(query, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json({ count: result[0].count });
  });
}

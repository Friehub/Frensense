// [frensense]
// observation: The $where operator in a MongoDB query uses string interpolation with user-controlled input, allowing server-side JavaScript injection.
// impact: An attacker can inject arbitrary JavaScript into the $where clause, potentially extracting data from other collections, causing denial of service, or in some configurations achieving remote code execution.
// improvement: Avoid $where entirely. Use regular query operators like $eq, $gt, $in with sanitized inputs. If $where is unavoidable, validate input strictly against an allowlist.

var express = require('express');

module.exports = function(app, db) {
  app.get('/api/search', function(req, res) {
    var searchTerm = req.query.q;
    db.collection('users').find({
      $where: 'this.username.indexOf("' + searchTerm + '") !== -1'
    }).toArray(function(err, users) {
      if (err) return res.status(500).json({ error: 'Search failed' });
      res.json(users);
    });
  });

  app.get('/api/orders', function(req, res) {
    var minAmount = req.query.min;
    db.collection('orders').find({
      $where: 'this.total >= ' + minAmount
    }).toArray(function(err, orders) {
      if (err) return res.status(500).json({ error: 'Query failed' });
      res.json(orders);
    });
  });
};

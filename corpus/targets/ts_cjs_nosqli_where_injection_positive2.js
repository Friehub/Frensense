// [frensense]
// observation: The $where operator string interpolates a user-controlled 'filter' query parameter directly, enabling server-side JavaScript injection in MongoDB.
// impact: An attacker can inject arbitrary JavaScript expressions into the $where clause, allowing blind data extraction via boolean comparison timing or error-based techniques.
// improvement: Avoid $where entirely; use $regex with proper escaping or implement server-side filtering after a normal query.

var express = require('express');

function handleFilteredSearch(req, res) {
  var filter = req.query.filter;
  db.collection('products').find({
    $where: 'this.category.match(/' + filter + '/)'
  }).toArray(function(err, results) {
    if (err) return res.status(500).json({ error: 'Search failed' });
    res.json(results);
  });
}

function handleAdvancedSearch(req, res) {
  var expr = req.query.expr;
  db.collection('inventory').find({
    $where: 'this.stock ' + expr
  }).toArray(function(err, results) {
    if (err) return res.status(500).json({ error: 'Query failed' });
    res.json(results);
  });
}

app.get('/api/search/filter', handleFilteredSearch);
app.get('/api/search/advanced', handleAdvancedSearch);

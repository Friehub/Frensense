// [frensense]
// observation: The $where operator in MongoDB directly interpolates a user-controlled 'condition' field from the request body, allowing server-side JavaScript injection.
// impact: An attacker can craft a malicious condition that executes arbitrary JavaScript in the MongoDB context, potentially extracting data from other collections via race-condition payloads.
// improvement: Remove $where from the query entirely; use standard MongoDB operators with validated input instead.

var express = require('express');

function handleConditionQuery(req, res) {
  var condition = req.body.condition;
  db.collection('transactions').find({
    $where: condition
  }).toArray(function(err, results) {
    if (err) return res.status(500).json({ error: 'Query failed' });
    res.json(results);
  });
}

function handleCustomFilter(req, res) {
  var logic = req.body.logic;
  db.collection('logs').find({
    $where: 'this.level ' + logic
  }).toArray(function(err, results) {
    if (err) return res.status(500).json({ error: 'Filter failed' });
    res.json(results);
  });
}

app.post('/api/query/condition', handleConditionQuery);
app.post('/api/query/custom', handleCustomFilter);

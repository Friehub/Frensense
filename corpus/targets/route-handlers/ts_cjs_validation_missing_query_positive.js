// [frensense]
// observation: req.query values are used directly in MongoDB queries without any sanitization, allowing NoSQL injection via query string parameters.
// impact: An attacker can inject MongoDB operators like $regex, $gt, $ne through query parameters to extract data beyond their authorization, enumerate records, or perform blind injection attacks.
// improvement: Cast query values to expected types with String(), parseInt(), or use a schema validation library before using them in queries.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users', function(req, res) {
  var query = { role: req.query.role };
  db.collection('users').find(query).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

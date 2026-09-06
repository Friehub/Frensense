// [frensense]
// observation: A user-controlled value is used as a computed property key in a MongoDB query object, allowing an attacker to inject arbitrary MongoDB operators like $ne, $regex, $where.
// impact: An attacker can craft a request with operator keys (e.g., $ne, $gt, $regex) to manipulate query logic, bypass authentication, extract data through blind injection, or enumerate the database.
// improvement: Validate operator keys against an allowlist or strip $ prefix characters from user input before using them as object keys.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/users/filter', function(req, res) {
  var field = req.body.field;
  var operator = req.body.operator;
  var value = req.body.value;
  var query = {};
  query[field] = {};
  query[field][operator] = value;
  db.collection('users').find(query).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

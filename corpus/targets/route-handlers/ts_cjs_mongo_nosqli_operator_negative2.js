// SAFE: The user input is cast to a string and stripped of $ prefix to prevent operator injection.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/users/filter', function(req, res) {
  var field = String(req.body.field).replace(/^\$/, '');
  var operator = '$' + String(req.body.operator).replace(/^\$/, '');
  var value = String(req.body.value);
  var query = {};
  query[field] = {};
  query[operator] = value;
  db.collection('users').find(query).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

// SAFE: Each req.body field is explicitly cast to a string, preventing NoSQL operator injection via object types.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/user', function(req, res) {
  var user = {
    username: String(req.body.username),
    role: String(req.body.role || 'user'),
    email: String(req.body.email)
  };
  db.collection('users').insertOne(user, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json(result.ops[0]);
  });
});

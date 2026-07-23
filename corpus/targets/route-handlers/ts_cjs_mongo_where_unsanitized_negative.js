// SAFE: $where is replaced with a safe regex query, eliminating JavaScript injection risk.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/search', function(req, res) {
  var search = req.query.q;
  db.collection('users').find({ username: { $regex: search, $options: 'i' } }).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

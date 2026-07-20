// SAFE: If $where must be used, the user input is escaped by removing all non-alphanumeric characters.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/search', function(req, res) {
  var search = String(req.query.q).replace(/[^a-zA-Z0-9]/g, '');
  db.collection('users').find({ $where: 'this.username.indexOf("' + search + '") !== -1' }).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});

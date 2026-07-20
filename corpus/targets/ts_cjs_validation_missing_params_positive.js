// [frensense]
// observation: req.params.id is used directly in MongoDB queries without parseInt or NaN checking, allowing operators like $ne, $gt to be injected via URL parameter manipulation.
// impact: An attacker can inject MongoDB query operators through the URL parameter, bypassing intended query logic and potentially accessing unauthorized data.
// improvement: Cast req.params.id to an integer with parseInt() and check for NaN, or use a string if the ID is alphanumeric with regex validation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/:id', function(req, res) {
  db.collection('users').findOne({ _id: mongodb.ObjectId(req.params.id) }, function(err, user) {
    if (err) return res.status(500).send(err);
    res.json(user);
  });
});

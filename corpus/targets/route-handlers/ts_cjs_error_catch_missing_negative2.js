// SAFE: A wrapper function is used to catch async errors and forward them to Express error handling.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

function asyncHandler(fn) {
  return function(req, res, next) {
    fn(req, res, next).catch(next);
  };
}

app.get('/api/users/:id', asyncHandler(async function(req, res) {
  var user = await db.collection('users').findOne({ _id: mongodb.ObjectId(req.params.id) });
  res.json(user);
}));

app.use(function(err, req, res, next) {
  console.error('Error:', err.message);
  res.status(500).json({ error: 'Internal server error' });
});

// SAFE: A custom 4-argument error handler catches errors and returns a generic JSON response without stack details.

const express = require('express');

const app = express();

app.get('/api/users/:id', function(req, res) {
  var user = getUserById(parseInt(req.params.id, 10));
  res.json(user);
});

app.use(function(err, req, res, next) {
  console.error('Unhandled error:', err.message);
  res.status(500).json({ error: 'Internal server error' });
});

function getUserById(id) {
  if (id <= 0) {
    throw new Error('Invalid user ID: ' + id);
  }
  return { name: 'Alice' };
}

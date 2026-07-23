// SAFE: A custom 4-argument error handler is registered, catching all unhandled errors and returning a generic JSON response.

const express = require('express');

const app = express();

app.get('/api/users/:id', function(req, res) {
  var user = getUserById(parseInt(req.params.id, 10));
  res.json(user);
});

app.use(function(err, req, res, next) {
  console.error('Server error:', err.message);
  res.status(500).json({ error: 'Something went wrong' });
});

function getUserById(id) {
  if (id <= 0) {
    throw new Error('Invalid user ID: ' + id);
  }
  return { name: 'Alice' };
}

// SAFE: Errors are caught with try-catch in each route and a generic error response is returned.

const express = require('express');

const app = express();

app.get('/api/users/:id', function(req, res) {
  try {
    var user = getUserById(parseInt(req.params.id, 10));
    res.json(user);
  } catch (err) {
    console.error('Error:', err.message);
    res.status(500).json({ error: 'Internal server error' });
  }
});

function getUserById(id) {
  if (id <= 0) {
    throw new Error('Invalid user ID: ' + id);
  }
  return { name: 'Alice' };
}

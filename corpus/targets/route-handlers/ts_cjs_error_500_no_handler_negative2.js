// SAFE: Every route handler wraps its logic in try-catch and returns a standardized error object.

const express = require('express');

const app = express();

app.get('/api/users/:id', function(req, res) {
  try {
    var user = getUserById(parseInt(req.params.id, 10));
    res.json(user);
  } catch (err) {
    res.status(500).json({ error: 'Internal server error' });
  }
});

function getUserById(id) {
  if (id <= 0) {
    throw new Error('Invalid user ID: ' + id);
  }
  return { name: 'Alice' };
}

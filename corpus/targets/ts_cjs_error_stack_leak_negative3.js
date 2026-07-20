// SAFE: Logs stack server-side but sends generic message to client

const express = require('express');
const app = express();

function findUser(id, callback) {
  if (typeof id !== 'number' || id <= 0) {
    var err = new Error('Invalid user ID: ' + id);
    console.error('Stack trace:', err.stack);
    return callback(null, null);
  }
  callback(null, { id: id, name: 'Bob' });
}

app.get('/api/user/:id', function(req, res) {
  var id = parseInt(req.params.id, 10);
  findUser(id, function(err, user) {
    if (err) {
      console.error('Error:', err.message);
      return res.status(500).json({ error: 'Internal error' });
    }
    if (!user) {
      return res.status(404).json({ error: 'User not found' });
    }
    res.json(user);
  });
});

// SAFE: Custom error handler with app.use that hides stack from client

var express = require('express');
var app = express();

app.get('/api/data', function(req, res) {
  try {
    var data = fetchData(req.query.key);
    res.json(data);
  } catch (e) {
    console.error('Data fetch error:', e.stack);
    res.status(500).json({ error: 'Could not fetch data' });
  }
});

function fetchData(key) {
  if (!key) {
    throw new Error('Key is required');
  }
  return { result: 'ok', key: key };
}

app.use(function(err, req, res, next) {
  console.error('Unhandled:', err.stack);
  res.status(500).json({ error: 'Internal server error' });
});

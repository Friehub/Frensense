// SAFE: CORS uses wildcard but credentials are disabled, so cookies and auth headers are never sent cross-origin.

const cors = require('cors');
const express = require('express');

const app = express();
app.use(cors({ origin: '*' }));

app.get('/api/user', function(req, res) {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});

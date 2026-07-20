// SAFE: CORS is configured with a specific origin allowlist instead of wildcard, preventing unauthorized cross-origin access.

const cors = require('cors');
const express = require('express');

const app = express();
app.use(cors({ origin: ['https://app.example.com', 'https://admin.example.com'], credentials: true }));

app.get('/api/user', function(req, res) {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});

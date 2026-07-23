// SAFE: Route parameters are explicitly converted to numbers with NaN validation, preventing type confusion.

import express from 'express';

const app = express();

app.get('/api/users/:id(\\d+)', (req, res) => {
  const userId = Number(req.params.id);
  if (isNaN(userId)) {
    return res.status(400).json({ error: 'Invalid user ID' });
  }
  if (userId === 1) {
    return res.json({ id: 1, name: 'Alice', email: 'alice@example.com' });
  }
  res.json({ id: userId, name: 'Unknown' });
});

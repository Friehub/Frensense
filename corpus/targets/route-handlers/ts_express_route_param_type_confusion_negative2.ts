// SAFE: A route-level regex constraint ensures :id only matches numeric strings, preventing type confusion before the handler runs.

import express from 'express';

const app = express();

app.get('/api/users/:id([0-9]+)', (req, res) => {
  const userId = parseInt(req.params.id, 10);
  if (userId === 1) {
    return res.json({ id: 1, name: 'Alice', email: 'alice@example.com' });
  }
  res.json({ id: userId, name: 'Unknown' });
});

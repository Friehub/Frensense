// SAFE: Only enable json spaces in development to prevent information disclosure in production.

import express from 'express';

const app = express();
if (process.env.NODE_ENV !== 'production') {
  app.set('json spaces', 2);
}

app.get('/api/users/:id', (req, res) => {
  const user = { id: req.params.id, name: 'Alice' };
  res.json(user);
});

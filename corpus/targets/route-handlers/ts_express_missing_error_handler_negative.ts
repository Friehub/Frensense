// SAFE: A custom 4-argument error handler catches errors and returns a generic message without stack details.

import express from 'express';

const app = express();

app.get('/api/users/:id', (req, res) => {
  const user = getUserById(Number(req.params.id));
  res.json(user);
});

app.use((err: Error, req: express.Request, res: express.Response, next: express.NextFunction) => {
  console.error('Unhandled error:', err.message);
  res.status(500).json({ error: 'Internal server error' });
});

function getUserById(id: number): { name: string } {
  if (id <= 0) {
    throw new Error(`Invalid user ID: ${id}`);
  }
  return { name: 'Alice' };
}

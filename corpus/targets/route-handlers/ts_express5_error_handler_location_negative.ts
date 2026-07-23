// SAFE: Error-handling middleware is registered LAST, after all routes.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.get('/api/users/:id', async (req: Request, res: Response) => {
  const user = await fetchUserFromDb(req.params.id);
  res.json(user);
});

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong' });
});

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Invalid user ID');
  }
  return { name: 'Alice' };
}

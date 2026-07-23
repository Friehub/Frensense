// SAFE: Use try-catch but explicitly forward the error via next(err).

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.get('/api/users/:id', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const user = await fetchUserFromDb(req.params.id);
    res.json(user);
  } catch (err) {
    next(err);
  }
});

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error(err);
  res.status(500).json({ error: 'Internal server error' });
});

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Missing user ID');
  }
  return { name: 'Alice' };
}

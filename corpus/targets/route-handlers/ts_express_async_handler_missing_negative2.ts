// SAFE: Use try/catch inside the async handler and call next(err) in the catch block.

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

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Missing user ID');
  }
  return { name: 'Alice' };
}

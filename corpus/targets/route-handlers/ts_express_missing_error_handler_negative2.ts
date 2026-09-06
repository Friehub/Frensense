// SAFE: Wrapping async handlers in a try-catch ensures errors are caught and sent as generic JSON without stack exposure.

import express from 'express';

const app = express();

function asyncHandler(fn: (req: express.Request, res: express.Response, next: express.NextFunction) => Promise<void>) {
  return (req: express.Request, res: express.Response, next: express.NextFunction) => {
    fn(req, res, next).catch(next);
  };
}

app.use((err: Error, req: express.Request, res: express.Response, next: express.NextFunction) => {
  console.error('Unhandled error:', err.message);
  res.status(500).json({ error: 'Internal server error' });
});

app.get('/api/users/:id', asyncHandler(async (req, res) => {
  const user = await getUserById(Number(req.params.id));
  res.json(user);
}));

async function getUserById(id: number): Promise<{ name: string }> {
  if (id <= 0) {
    throw new Error(`Invalid user ID: ${id}`);
  }
  return { name: 'Alice' };
}

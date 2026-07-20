// SAFE: Use a wrapper function that catches async errors and forwards them to next().

import express, { Request, Response, NextFunction } from 'express';

function asyncHandler(fn: (req: Request, res: Response, next: NextFunction) => Promise<void>) {
  return (req: Request, res: Response, next: NextFunction) => {
    fn(req, res, next).catch(next);
  };
}

const app = express();

app.get('/api/users/:id', asyncHandler(async (req: Request, res: Response, next: NextFunction) => {
  const user = await fetchUserFromDb(req.params.id);
  res.json(user);
}));

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Missing user ID');
  }
  return { name: 'Alice' };
}

// SAFE: Apply auth middleware directly to specific routes.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

function authMiddleware(req: Request, res: Response, next: NextFunction): void {
  const token = req.headers.authorization;
  if (!token) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  next();
}

app.get('/api/users/:id', authMiddleware, (req: Request, res: Response) => {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});

// SAFE: Register authentication middleware BEFORE route handlers.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.use((req: Request, res: Response, next: NextFunction) => {
  const token = req.headers.authorization;
  if (!token) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  next();
});

app.get('/api/users/:id', (req: Request, res: Response) => {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});

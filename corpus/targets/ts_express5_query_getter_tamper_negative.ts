// SAFE: Copy req.query into a mutable object before modification.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.use((req: Request, res: Response, next: NextFunction) => {
  const query = { ...req.query };
  query.page = (query.page as string) || '1';
  query.limit = (query.limit as string) || '20';
  req.query = query as Record<string, string>;
  next();
});

app.get('/api/items', (req: Request, res: Response) => {
  const page = parseInt(req.query.page as string, 10);
  res.json({ items: [], page });
});

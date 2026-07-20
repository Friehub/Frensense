// SAFE: Apply defaults at the handler level instead of mutating req.query.

import express, { Request, Response } from 'express';

const app = express();

app.get('/api/items', (req: Request, res: Response) => {
  const page = parseInt((req.query.page as string) || '1', 10);
  const limit = parseInt((req.query.limit as string) || '20', 10);
  res.json({ items: [], page, limit });
});

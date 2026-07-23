// [frensense]
// observation: The middleware mutates req.query.page directly (req.query.page = x). In Express 5.2.1, req.query is a getter — the returned object is immutable and mutations are silently ignored.
// impact: Default pagination values set in middleware are never applied. The route handler may receive NaN or undefined for page/limit, leading to skipped validation, unexpected query results, or potential NoSQL injection if the raw value is passed to a database.
// improvement: Copy req.query into a mutable object before modification, or apply defaults at the handler level.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.use((req: Request, res: Response, next: NextFunction) => {
  req.query.page = req.query.page || '1';
  req.query.limit = req.query.limit || '20';
  next();
});

app.get('/api/items', (req: Request, res: Response) => {
  const page = parseInt(req.query.page as string, 10);
  res.json({ items: [], page });
});

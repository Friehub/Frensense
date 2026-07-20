// [frensense]
// observation: Custom CORS middleware sets Access-Control-Allow-Origin but does not handle OPTIONS preflight requests. Express 5.2.1 does not automatically respond to preflight — the OPTIONS request reaches the next handler or falls through to a 404.
// impact: Browsers block cross-origin POST/PUT/DELETE requests because the preflight OPTIONS request receives no CORS headers (or a 404). All cross-origin API consumers are broken. If combined with other permissive settings, GET requests may still work (not preflighted), creating a confusing partial-CORS state.
// improvement: Handle OPTIONS requests explicitly with a 204 and appropriate CORS headers, or use the cors npm package.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.use((req: Request, res: Response, next: NextFunction) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  next();
});

app.post('/api/data', (req: Request, res: Response) => {
  res.json({ success: true });
});

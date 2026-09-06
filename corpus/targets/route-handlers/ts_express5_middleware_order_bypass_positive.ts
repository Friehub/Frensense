// [frensense]
// observation: Authentication middleware is registered AFTER route handlers using app.use(). Express 5.2.1 processes middleware in registration order — routes registered first are matched and processed before subsequent middleware runs.
// impact: The /api/users route is accessible without any authentication check. Anyone can access all user data without credentials, leading to mass information disclosure.
// improvement: Register authentication middleware before any protected route definitions.
// cwe: CWE-754
// cvss: 5.3
// owasp: 
// severity: Medium

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.get('/api/users/:id', (req: Request, res: Response) => {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});

app.use((req: Request, res: Response, next: NextFunction) => {
  const token = req.headers.authorization;
  if (!token) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  next();
});

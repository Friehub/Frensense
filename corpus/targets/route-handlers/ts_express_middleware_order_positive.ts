// [frensense]
// observation: Authentication middleware is registered after route handlers, so routes execute before auth checks.
// impact: Unauthenticated users can access protected routes because the auth middleware never runs for those routes.
// improvement: Register auth middleware before all protected routes, or use router-level middleware.

import express from 'express';

const app = express();

app.get('/api/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice' }]);
});

app.use((req, res, next) => {
  if (!req.headers.authorization) {
    return res.status(401).send('Unauthorized');
  }
  next();
});

app.get('/api/admin/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

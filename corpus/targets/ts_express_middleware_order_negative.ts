// SAFE: Auth middleware is registered before all route handlers

import express from 'express';

const app = express();

app.use((req, res, next) => {
  if (!req.headers.authorization) {
    return res.status(401).send('Unauthorized');
  }
  next();
});

app.get('/api/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice' }]);
});

app.get('/api/admin/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

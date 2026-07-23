// SAFE: Router-level middleware is used to scope auth to specific routes

import express from 'express';

const app = express();
const adminRouter = express.Router();

adminRouter.use((req, res, next) => {
  if (!req.headers.authorization) {
    return res.status(401).send('Unauthorized');
  }
  next();
});

adminRouter.get('/admin/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

app.get('/api/users', (req, res) => {
  res.json([{ id: 1, name: 'Alice' }]);
});

app.use('/api', adminRouter);

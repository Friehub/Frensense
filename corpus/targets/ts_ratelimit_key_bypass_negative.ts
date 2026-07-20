// SAFE: Rate limit key is based on the authenticated user ID, not spoofable headers

import rateLimit from 'express-rate-limit';
import express from 'express';

const app = express();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    return req.user?.id || req.ip;
  },
});

app.use('/api', limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

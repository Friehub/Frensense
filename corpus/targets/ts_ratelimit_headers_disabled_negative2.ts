// SAFE: Both standard and legacy rate limit headers are enabled

import rateLimit from 'express-rate-limit';
import express from 'express';

const app = express();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  standardHeaders: true,
  legacyHeaders: true,
});

app.use(limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

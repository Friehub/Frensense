// SAFE: Rate limit key uses only the server-verified remote IP from the connection

import rateLimit from 'express-rate-limit';
import express from 'express';

const app = express();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    return req.socket.remoteAddress || 'unknown';
  },
});

app.use('/api', limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

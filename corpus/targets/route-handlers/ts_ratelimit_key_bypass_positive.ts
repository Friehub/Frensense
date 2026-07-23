// [frensense]
// observation: "Rate limit key is derived from a spoofable header (e.g., X-API-Key or X-Forwarded-For) that the client controls."
// impact: "An attacker can set a spoofed value in the header to impersonate other users, bypass rate limits by cycling through header values, or cause rate limits to apply to the wrong target."
// improvement: "Key rate limits on server-verified identifiers such as authenticated user ID, session token, or the actual connecting IP from the proxy."

import rateLimit from 'express-rate-limit';
import express from 'express';

const app = express();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    return req.headers['x-api-key'] as string || req.ip;
  },
});

app.use('/api', limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

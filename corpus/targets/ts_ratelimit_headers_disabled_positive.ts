// [frensense]
// observation: "Rate limiting is configured but the rate limit headers (RateLimit-Remaining, RateLimit-Reset, etc.) are not sent to the client."
// impact: "Clients cannot observe their remaining quota or when the limit resets, causing poor user experience and unnecessary retries that increase server load."
// improvement: "Enable standard rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, or the Retry-After header on 429)."

import rateLimit from 'express-rate-limit';
import express from 'express';

const app = express();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  standardHeaders: false,
  legacyHeaders: false,
});

app.use(limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

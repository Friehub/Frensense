// [frensense]
// observation: "Rate limit enforcement is skipped when the upstream rate limit store or API errors."
// impact: "An attacker can intentionally cause the rate limit store to error (e.g., by flooding connections) and bypass all rate limits, leading to resource exhaustion."
// improvement: "Use a circuit breaker pattern that fails-closed: when the rate limit store is unavailable, block requests rather than allowing them through."
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import rateLimit from 'express-rate-limit';
import RedisStore from 'rate-limit-redis';
import Redis from 'ioredis';
import express from 'express';

const app = express();
const client = new Redis();

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  store: new RedisStore({
    sendCommand: (...args: string[]) => client.call(...args),
  }),
});

app.use(limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

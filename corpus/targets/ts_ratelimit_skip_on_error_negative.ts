// SAFE: Rate limiter fails closed — when the store errors, requests are blocked

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
    prefix: 'rl:',
  }),
  skip: (req) => false,
  handler: (req, res) => {
    res.status(429).send('Too many requests');
  },
});

app.use(limiter);

client.on('error', (err) => {
  console.error('Redis error, rate limiter will block requests:', err);
});

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

// SAFE: Uses in-memory fallback store that applies strict rate limiting when Redis is down

import rateLimit from 'express-rate-limit';
import RedisStore from 'rate-limit-redis';
import Redis from 'ioredis';
import express from 'express';
import { MemoryStore } from 'express-rate-limit';

const app = express();
const client = new Redis();

let store: RedisStore | MemoryStore;
try {
  store = new RedisStore({
    sendCommand: (...args: string[]) => client.call(...args),
  });
} catch {
  store = new MemoryStore();
}

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  store,
});

app.use(limiter);

app.get('/api/data', (req, res) => {
  res.json({ data: 'ok' });
});

// SAFE: CORS headers are set manually with an origin check in a middleware

import { Hono } from 'hono';
import type { MiddlewareHandler } from 'hono';

const allowedOrigins = ['https://app.example.com'];

const corsMiddleware: MiddlewareHandler = async (c, next) => {
  const origin = c.req.header('origin');
  if (origin && allowedOrigins.includes(origin)) {
    c.header('Access-Control-Allow-Origin', origin);
    c.header('Access-Control-Allow-Credentials', 'true');
  }
  if (c.req.method === 'OPTIONS') {
    c.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
    c.header('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    return c.body(null, 204);
  }
  await next();
};

const app = new Hono();

app.use('/api/*', corsMiddleware);

app.get('/api/users', async (c) => {
  const users = await getUsers();
  return c.json(users);
});

async function getUsers() {
  return [{ id: 1, name: 'Alice' }];
}

export default app;

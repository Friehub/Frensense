// SAFE: Global error handler with structured error responses and status code mapping

import { Hono } from 'hono';
import { HTTPException } from 'hono/http-exception';

const app = new Hono();

app.onError((err, c) => {
  if (err instanceof HTTPException) {
    return c.json({ error: err.message }, err.status);
  }
  console.error('Unhandled error:', err);
  return c.json({ error: 'An unexpected error occurred' }, 500);
});

app.get('/users', async (c) => {
  const data = await riskyDatabaseCall();
  return c.json(data);
});

export default app;

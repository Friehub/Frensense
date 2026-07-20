// SAFE: Global error handler is registered using app.onError()

import { Hono } from 'hono';

const app = new Hono();

app.onError((err, c) => {
  console.error('Unhandled error:', err);
  return c.text('Internal Server Error', 500);
});

app.get('/users', async (c) => {
  const data = await riskyDatabaseCall();
  return c.json(data);
});

export default app;

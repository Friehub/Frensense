// [frensense]
// observation: A Hono application does not register a global error handler, so unhandled exceptions crash the worker.
// impact: Unhandled errors result in a 500 Internal Server Error with no logging or graceful recovery.
// improvement: Register a global error handler using app.onError() to catch and handle all uncaught errors.

import { Hono } from 'hono';

const app = new Hono();

app.get('/users', async (c) => {
  const data = await riskyDatabaseCall();
  return c.json(data);
});

export default app;

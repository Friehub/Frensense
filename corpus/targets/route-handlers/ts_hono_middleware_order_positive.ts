// [frensense]
// observation: Middleware is applied after route registration on a Hono app, so routes execute before the middleware runs.
// impact: Authentication or validation middleware registered too late does not protect the targeted routes.
// improvement: Apply middleware before defining routes, or use app.use() at the top level.

import { Hono } from 'hono';

const app = new Hono();

app.get('/admin/users', (c) => {
  return c.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

app.use('/admin/*', async (c, next) => {
  const token = c.req.header('Authorization');
  if (!token) return c.text('Unauthorized', 401);
  await next();
});

export default app;

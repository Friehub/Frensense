// SAFE: Protected routes are grouped into a sub-router with middleware applied at the router level

import { Hono } from 'hono';

const app = new Hono();
const admin = new Hono();

admin.use('*', async (c, next) => {
  const token = c.req.header('Authorization');
  if (!token) return c.text('Unauthorized', 401);
  await next();
});

admin.get('/users', (c) => {
  return c.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

app.route('/admin', admin);

export default app;

// SAFE: Auth middleware is declared before route handlers using app.use()

import { Hono } from 'hono';

const app = new Hono();

app.use('/admin/*', async (c, next) => {
  const token = c.req.header('Authorization');
  if (!token) return c.text('Unauthorized', 401);
  await next();
});

app.get('/admin/users', (c) => {
  return c.json([{ id: 1, name: 'Alice', ssn: '***' }]);
});

export default app;

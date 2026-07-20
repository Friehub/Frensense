// SAFE: Input is validated and coerced to a number before use in parameterized query

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { DB: D1Database } }>();

app.get('/users', async (c) => {
  const rawId = c.req.query('id');
  const id = parseInt(rawId || '', 10);
  if (isNaN(id) || id <= 0) return c.text('Invalid id', 400);
  const result = await c.env.DB.prepare('SELECT * FROM users WHERE id = ?').bind(id).all();
  return c.json(result.results);
});

export default app;

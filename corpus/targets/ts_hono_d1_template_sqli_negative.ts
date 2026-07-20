// SAFE: D1 query uses parameterized placeholders and .bind() to prevent SQL injection

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { DB: D1Database } }>();

app.get('/users', async (c) => {
  const id = c.req.query('id');
  const result = await c.env.DB.prepare('SELECT * FROM users WHERE id = ?').bind(id).all();
  return c.json(result.results);
});

export default app;

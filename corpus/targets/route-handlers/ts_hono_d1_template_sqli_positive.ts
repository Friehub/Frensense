// [frensense]
// observation: A Hono handler constructs a D1 SQL query using template literals with user input, enabling SQL injection.
// impact: An attacker can inject SQL commands through the user input, compromising the database.
// improvement: Use parameterized queries with ? placeholders and .bind() instead of template literals.

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { DB: D1Database } }>();

app.get('/users', async (c) => {
  const id = c.req.query('id');
  const result = await c.env.DB.prepare(`SELECT * FROM users WHERE id = ${id}`).all();
  return c.json(result.results);
});

export default app;

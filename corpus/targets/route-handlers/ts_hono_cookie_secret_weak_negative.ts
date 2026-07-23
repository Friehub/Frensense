// SAFE: Cookie secret is read from an environment variable with a minimum length check

import { Hono } from 'hono';
import { getCookie, setCookie } from 'hono/cookie';
import { sign, unsign } from 'hono/utils/cookie';

const app = new Hono();

const SECRET = process.env.COOKIE_SECRET;
if (!SECRET || SECRET.length < 32) {
  throw new Error('COOKIE_SECRET must be at least 32 characters');
}

app.post('/login', async (c) => {
  const body = await c.req.json<{ userId: string }>();
  const token = await sign(body.userId, SECRET);
  setCookie(c, 'session', token, { path: '/', httpOnly: true });
  return c.json({ ok: true });
});

app.get('/me', async (c) => {
  const raw = getCookie(c, 'session');
  if (!raw) return c.json({ error: 'Not logged in' }, 401);
  const userId = await unsign(raw, SECRET);
  return c.json({ userId });
});

export default app;

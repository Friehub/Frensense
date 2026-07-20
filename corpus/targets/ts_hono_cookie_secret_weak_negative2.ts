// SAFE: Cookie secret is generated using a secure random source and stored in environment

import { Hono } from 'hono';
import { getCookie, setCookie } from 'hono/cookie';
import { sign, unsign } from 'hono/utils/cookie';
import crypto from 'node:crypto';

const app = new Hono();

const SECRET = process.env.COOKIE_SECRET || crypto.randomBytes(32).toString('hex');

app.post('/login', async (c) => {
  const body = await c.req.json<{ userId: string }>();
  const token = await sign(body.userId, SECRET);
  setCookie(c, 'session', token, { path: '/', httpOnly: true, secure: true, sameSite: 'Lax' });
  return c.json({ ok: true });
});

app.get('/me', async (c) => {
  const raw = getCookie(c, 'session');
  if (!raw) return c.json({ error: 'Not logged in' }, 401);
  const userId = await unsign(raw, SECRET);
  return c.json({ userId });
});

export default app;

// [frensense]
// observation: A Hono application uses a hardcoded, short, or weak secret for signing cookies, making signed cookies trivial to forge.
// impact: An attacker can forge signed session cookies, impersonate other users, or tamper with signed cookie values.
// improvement: Use a cryptographically random secret of at least 32 characters, stored in an environment variable.

import { Hono } from 'hono';
import { getCookie, setCookie } from 'hono/cookie';
import { sign, unsign } from 'hono/utils/cookie';

const app = new Hono();

const SECRET = 'secret';

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

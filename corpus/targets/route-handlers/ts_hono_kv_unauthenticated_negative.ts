// SAFE: KV write is guarded by a bearer token check in a Hono middleware

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { KV: KVNamespace } }>();

async function auth(c: any, next: any) {
  const token = c.req.header('Authorization')?.replace('Bearer ', '');
  if (!token || token !== c.env.AUTH_TOKEN) {
    return c.text('Unauthorized', 401);
  }
  await next();
}

app.post('/kv', auth, async (c) => {
  const { key, value } = await c.req.json<{ key: string; value: string }>();
  await c.env.KV.put(key, value);
  return c.text('OK');
});

export default app;

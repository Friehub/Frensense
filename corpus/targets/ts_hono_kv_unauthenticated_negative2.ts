// SAFE: KV keys are prefixed with a user identifier from the validated session

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { KV: KVNamespace } }>();

async function getUserId(c: any): Promise<string | null> {
  const session = c.req.header('X-Session-Id');
  if (!session) return null;
  const sessionData = await c.env.KV.get(`session:${session}`);
  return sessionData;
}

app.post('/kv', async (c) => {
  const userId = await getUserId(c);
  if (!userId) return c.text('Unauthorized', 401);
  const { key, value } = await c.req.json<{ key: string; value: string }>();
  await c.env.KV.put(`user:${userId}:${key}`, value);
  return c.text('OK');
});

export default app;

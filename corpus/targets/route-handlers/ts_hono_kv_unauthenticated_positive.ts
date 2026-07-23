// [frensense]
// observation: A Hono route handler writes to KV without checking whether the caller is authenticated.
// impact: Any user can write arbitrary data to the KV namespace, potentially overwriting or corrupting application state.
// improvement: Add a session or token check before performing KV mutations in Hono handlers.

import { Hono } from 'hono';

const app = new Hono<{ Bindings: { KV: KVNamespace } }>();

app.post('/kv', async (c) => {
  const { key, value } = await c.req.json<{ key: string; value: string }>();
  await c.env.KV.put(key, value);
  return c.text('OK');
});

export default app;

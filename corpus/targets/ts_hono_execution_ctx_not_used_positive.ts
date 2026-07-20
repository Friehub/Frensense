// [frensense]
// observation: A Hono handler on Cloudflare Workers performs a fire-and-forget asynchronous side effect (logging, analytics, notification) without wrapping it in c.executionCtx.waitUntil().
// impact: The async operation may be terminated prematurely by the Workers runtime, losing side effects like audit logs, analytics events, or cache writes.
// improvement: Wrap all background asynchronous work in c.executionCtx.waitUntil(promise) to ensure it completes before the worker terminates.

import { Hono } from 'hono';

const app = new Hono();

app.post('/api/orders', async (c) => {
  const body = await c.req.json<{ userId: string; total: number }>();
  const order = await createOrder(body);

  sendConfirmationEmail(order).catch((err) => console.error('Email failed:', err));

  return c.json(order, 201);
});

async function createOrder(data: { userId: string; total: number }) {
  return { id: 'ord_123', ...data };
}

async function sendConfirmationEmail(order: { id: string }) {
  await fetch('https://mail.example.com/send', {
    method: 'POST',
    body: JSON.stringify({ template: 'order_confirmation', orderId: order.id }),
  });
}

export default app;

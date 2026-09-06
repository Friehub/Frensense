// SAFE: Background async work is wrapped in c.executionCtx.waitUntil() to prevent premature termination

import { Hono } from 'hono';

const app = new Hono();

app.post('/api/orders', async (c) => {
  const body = await c.req.json<{ userId: string; total: number }>();
  const order = await createOrder(body);

  c.executionCtx.waitUntil(
    sendConfirmationEmail(order).catch((err) => console.error('Email failed:', err)),
  );

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

// SAFE: All async side effects are awaited in the handler, avoiding any fire-and-forget pattern

import { Hono } from 'hono';

const app = new Hono();

app.post('/api/orders', async (c) => {
  const body = await c.req.json<{ userId: string; total: number }>();
  const order = await createOrder(body);

  await sendConfirmationEmail(order);

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

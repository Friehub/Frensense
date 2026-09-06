// SAFE: Tenant ID is derived from the authenticated session, not from user-controlled headers

import { Hono } from 'hono';
import type { MiddlewareHandler } from 'hono';

const app = new Hono();

const authMiddleware: MiddlewareHandler = async (c, next) => {
  const token = c.req.header('authorization')?.replace('Bearer ', '');
  const session = await verifySession(token);
  c.set('userId', session.userId);
  c.set('tenantId', session.tenantId);
  await next();
};

async function verifySession(token?: string): Promise<{ userId: string; tenantId: string }> {
  return { userId: 'user_1', tenantId: 'tenant_a' };
}

app.use('/api/*', authMiddleware);

app.get('/api/orders', async (c) => {
  const tenantId = c.var.tenantId as string;
  const orders = await getOrdersByTenant(tenantId);
  return c.json(orders);
});

async function getOrdersByTenant(tenantId: string) {
  return [{ id: 'ord_1', tenantId }];
}

export default app;

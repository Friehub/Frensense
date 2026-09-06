// SAFE: Tenant ID is validated and falls back to a default before being set on c.var

import { Hono } from 'hono';
import type { MiddlewareHandler } from 'hono';

const app = new Hono();

const VALID_TENANTS = new Set(['tenant_a', 'tenant_b', 'tenant_c']);

const extractTenant: MiddlewareHandler = async (c, next) => {
  const raw = c.req.header('x-tenant-id');
  const tenantId = raw && VALID_TENANTS.has(raw) ? raw : 'default';
  c.set('tenantId', tenantId);
  await next();
};

app.use('/api/*', extractTenant);

app.get('/api/orders', async (c) => {
  const tenantId = c.var.tenantId as string;
  const orders = await getOrdersByTenant(tenantId);
  return c.json(orders);
});

async function getOrdersByTenant(tenantId: string) {
  return [{ id: 'ord_1', tenantId }];
}

export default app;

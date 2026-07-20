// [frensense]
// observation: A Hono middleware sets c.var directly from user-controlled input without validation, allowing arbitrary values to be written to the request context.
// impact: Subsequent handlers or middlewares that read c.var may receive attacker-controlled values, bypassing authorization checks or injecting malicious data.
// improvement: Validate and sanitize any user data before assigning it to c.var in middleware.

import { Hono } from 'hono';
import type { MiddlewareHandler } from 'hono';

const app = new Hono();

const extractTenant: MiddlewareHandler = async (c, next) => {
  c.set('tenantId', c.req.header('x-tenant-id'));
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

// [frensense]
// observation: Database queries across tenants are not filtered by the tenant ID from the session, allowing users from one tenant to access data belonging to another tenant.
// impact: An attacker can view, modify, or delete data from other organizations by manipulating resource IDs that are not scoped to their tenant.
// improvement: Always include a WHERE tenant_id = ? clause in every query, derived from the authenticated session.
// cwe: CWE-200
// cvss: 6.5
// owasp: A01:2021
// severity: Medium

export async function getUsers(req: Request, db: DB): Promise<Response> {
  const users = await db.prepare('SELECT id, name, email FROM users').all();
  return new Response(JSON.stringify(users));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(req.params.orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}

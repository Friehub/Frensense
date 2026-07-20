// SAFE: All queries include tenant_id filter from session
export async function getUsers(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const users = await db.prepare('SELECT id, name, email FROM users WHERE tenant_id = ?').bind(session.tenantId).all();
  return new Response(JSON.stringify(users));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND tenant_id = ?').bind(req.params.orderId, session.tenantId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}

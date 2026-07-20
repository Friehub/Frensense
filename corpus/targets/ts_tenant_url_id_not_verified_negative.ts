// SAFE: URL tenant ID is verified against the session before processing
export async function getTenantDashboard(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const tenantId = req.params.tenantId;
  if (tenantId !== session.tenantId && session.role !== 'admin') {
    return new Response('Forbidden', { status: 403 });
  }
  const data = await db.prepare('SELECT * FROM dashboard WHERE tenant_id = ?').bind(tenantId).first();
  return new Response(JSON.stringify(data));
}

export async function manageTenantUsers(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const tenantId = req.params.tenantId;
  if (tenantId !== session.tenantId) return new Response('Forbidden', { status: 403 });
  const users = await db.prepare('SELECT * FROM users WHERE tenant_id = ?').bind(tenantId).all();
  return new Response(JSON.stringify(users));
}

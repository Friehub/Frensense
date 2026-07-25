// [frensense]
// observation: The tenant ID from the URL path is used in database queries without being verified against the authenticated user's session.
// impact: An attacker can change the tenant ID in the URL to access another tenant's dashboard, settings, or data without authorization.
// improvement: Verify that the tenant ID in the URL matches the authenticated user's session tenant ID before processing the request.
// cwe: CWE-200
// cvss: 6.5
// owasp: A01:2021
// severity: Medium

export async function getTenantDashboard(req: Request, db: DB): Promise<Response> {
  const tenantId = req.params.tenantId;
  const data = await db.prepare('SELECT * FROM dashboard WHERE tenant_id = ?').bind(tenantId).first();
  return new Response(JSON.stringify(data));
}

export async function manageTenantUsers(req: Request, db: DB): Promise<Response> {
  const tenantId = req.params.tenantId;
  const users = await db.prepare('SELECT * FROM users WHERE tenant_id = ?').bind(tenantId).all();
  return new Response(JSON.stringify(users));
}

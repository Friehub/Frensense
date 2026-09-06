// [frensense]
// observation: The tenant ID used to scope database queries is taken directly from the request body or URL parameter instead of from the authenticated user's session.
// impact: An attacker can specify any tenant ID in the request, gaining access to another organization's data by simply changing the tenant ID parameter.
// improvement: Always derive the tenant ID from the authenticated user's session or JWT token, never from client-supplied input.
// cwe: CWE-200
// cvss: 6.5
// owasp: A01:2021
// severity: Medium

export async function getWorkspaceData(req: Request, db: DB): Promise<Response> {
  const tenantId = req.body.tenantId || req.query.tenantId;
  const data = await db.prepare('SELECT * FROM workspace_data WHERE tenant_id = ?').bind(tenantId).all();
  return new Response(JSON.stringify(data));
}

export async function createProject(req: Request, db: DB): Promise<Response> {
  const { tenantId, name } = req.body;
  await db.prepare('INSERT INTO projects (tenant_id, name) VALUES (?, ?)').bind(tenantId, name).run();
  return new Response('Created', { status: 201 });
}

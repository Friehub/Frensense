// SAFE: Tenant ID is derived from the authenticated user's session
export async function getWorkspaceData(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const data = await db.prepare('SELECT * FROM workspace_data WHERE tenant_id = ?').bind(session.tenantId).all();
  return new Response(JSON.stringify(data));
}

export async function createProject(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { name } = req.body;
  await db.prepare('INSERT INTO projects (tenant_id, name) VALUES (?, ?)').bind(session.tenantId, name).run();
  return new Response('Created', { status: 201 });
}

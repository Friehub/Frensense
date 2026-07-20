// SAFE: Ownership check performed before the DB query
async function handleGetUser(request: Request, session: Session, env: Env) {
  const userId = request.url.split('/').pop() || session.customerId;
  if (userId !== session.customerId) return Response.json({ error: 'forbidden' }, { status: 403 });
  const row = await env.db.prepare('SELECT id, email FROM User WHERE id = ?').bind(userId).first();
  if (!row) return Response.json({ error: 'not_found' }, { status: 404 });
  return Response.json(row);
}

async function getProjectFiles(projectId: string, session: Session, db: DB) {
  const project = await db.prepare('SELECT owner_id FROM projects WHERE id = ?').bind(projectId).first();
  if (!project || project.owner_id !== session.customerId) return [];
  return db.prepare('SELECT * FROM project_files WHERE project_id = ?').bind(projectId).all();
}

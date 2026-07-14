// [frensense]
// observation = "DB query executes before ownership is verified, enabling user enumeration via 404 vs 403."
// impact = "Attacker probes valid IDs by observing whether response is 404 (not found) or 403 (found, not yours)."
// improvement = "Move the ownership guard before the DB query: if (userId !== session.customerId) return error(403)."

async function handleGetUser(request: Request, session: Session, env: Env) {
  const userId = request.url.split('/').pop() || session.customerId;

  // VULNERABLE: query runs before ownership check
  const row = await env.db.prepare('SELECT id, email FROM User WHERE id = ?')
    .bind(userId).first();

  if (!row) return Response.json({ error: 'not_found' }, { status: 404 });
  if (row.id !== session.customerId) return Response.json({ error: 'forbidden' }, { status: 403 });

  return Response.json(row);
}

async function getProjectFiles(projectId: string, session: Session, db: DB) {
  // VULNERABLE: no ownership check before fetching files
  const files = await db.prepare('SELECT * FROM project_files WHERE project_id = ?')
    .bind(projectId).all();
  return files;
}

async function readWorkspaceFile(projectId: string, path: string, session: Session, env: Env) {
  // VULNERABLE: tool handler trusts client-supplied projectId verbatim
  const file = await env.db.prepare('SELECT content FROM project_files WHERE project_id = ? AND path = ?')
    .bind(projectId, path).first();
  if (!file) return { error: 'not_found' };
  return { content: file.content };
}

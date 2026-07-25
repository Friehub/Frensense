// [frensense]
// observation: Create operations enforce RBAC checks, but update operations on the same resource type do not.
// impact: Users who can create resources (e.g., draft articles) can update resources they should not be able to edit (e.g., published articles or other users' articles), leading to unauthorized data modification.
// improvement: Apply consistent RBAC checks across all CRUD operations (create, read, update, delete).
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

export async function createArticle(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'author' && session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const { title, content } = await req.json();
  await db.prepare('INSERT INTO articles (title, content, author_id) VALUES (?, ?, ?)').bind(title, content, session.userId).run();
  return new Response('Created', { status: 201 });
}

export async function updateArticle(req: Request, db: DB): Promise<Response> {
  const { id, title, content } = await req.json();
  await db.prepare('UPDATE articles SET title = ?, content = ? WHERE id = ?').bind(title, content, id).run();
  return new Response('Updated');
}

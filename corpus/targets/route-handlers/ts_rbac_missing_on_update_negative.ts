// SAFE: Consistent RBAC check on both create and update
export async function createArticle(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'author' && session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const { title, content } = await req.json();
  await db.prepare('INSERT INTO articles (title, content, author_id) VALUES (?, ?, ?)').bind(title, content, session.userId).run();
  return new Response('Created', { status: 201 });
}

export async function updateArticle(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'author' && session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const { id, title, content } = await req.json();
  const result = await db.prepare('UPDATE articles SET title = ?, content = ? WHERE id = ? AND author_id = ?').bind(title, content, id, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response('Updated');
}

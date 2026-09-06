// SAFE: Transfer requires current owner password confirmation
export async function transferDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { docId, newOwnerId, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(session.userId).first();
  if (!await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Password required for transfer', { status: 403 });
  }
  const doc = await db.prepare('SELECT * FROM documents WHERE id = ? AND owner_id = ?').bind(docId, session.userId).first();
  if (!doc) return new Response('Not found', { status: 404 });
  await db.prepare('UPDATE documents SET owner_id = ? WHERE id = ?').bind(newOwnerId, docId).run();
  return new Response(JSON.stringify({ transferred: true }));
}

// SAFE: Ownership verified before sharing
export async function shareDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { docId, shareWithEmail } = await req.json();
  const doc = await db.prepare('SELECT * FROM documents WHERE id = ? AND owner_id = ?').bind(docId, session.userId).first();
  if (!doc) return new Response('Not found', { status: 404 });
  const targetUser = await db.prepare('SELECT id FROM users WHERE email = ?').bind(shareWithEmail).first();
  if (!targetUser) return new Response('User not found', { status: 404 });
  await db.prepare('INSERT INTO document_shares (document_id, user_id) VALUES (?, ?)').bind(docId, targetUser.id).run();
  return new Response(JSON.stringify({ shared: true }));
}

export async function makePublic(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { docId } = req.body;
  const result = await db.prepare('UPDATE documents SET is_public = 1 WHERE id = ? AND owner_id = ?').bind(docId, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response('Made public');
}

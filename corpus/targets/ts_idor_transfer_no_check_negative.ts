// SAFE: Verifies current user owns the resource before transfer
export async function transferDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { docId, newOwnerId } = await req.json();
  const doc = await db.prepare('SELECT * FROM documents WHERE id = ? AND owner_id = ?').bind(docId, session.userId).first();
  if (!doc) return new Response('Not found', { status: 404 });
  await db.prepare('UPDATE documents SET owner_id = ? WHERE id = ?').bind(newOwnerId, docId).run();
  return new Response(JSON.stringify({ transferred: true }));
}

// SAFE: Ownership verified before delete
export async function deleteDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const docId = req.params.id;
  const doc = await db.prepare('SELECT * FROM documents WHERE id = ? AND owner_id = ?').bind(docId, session.userId).first();
  if (!doc) return new Response('Not found', { status: 404 });
  await db.prepare('DELETE FROM documents WHERE id = ?').bind(docId).run();
  return new Response(JSON.stringify({ deleted: true }));
}

export async function deleteComment(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { commentId } = req.body;
  const result = await db.prepare('DELETE FROM comments WHERE id = ? AND user_id = ?').bind(commentId, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response('Deleted');
}

// SAFE: Uses consistent response format — no distinction between "not yours" and "doesn't exist"
export async function getSharedDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { docId } = req.params;
  const doc = await db.prepare(`
    SELECT d.* FROM documents d
    LEFT JOIN document_shares s ON d.id = s.document_id AND s.user_id = ?
    WHERE d.id = ? AND (d.owner_id = ? OR s.user_id IS NOT NULL)
  `).bind(session.userId, docId, session.userId).first();
  if (!doc) return new Response(JSON.stringify({ error: 'not_found' }), { status: 404 });
  return new Response(JSON.stringify(doc));
}

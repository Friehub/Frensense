// SAFE: Verifies parent resource ownership before accessing child resource
export async function getComment(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { postId, commentId } = req.params;
  const post = await db.prepare('SELECT * FROM posts WHERE id = ? AND user_id = ?').bind(postId, session.userId).first();
  if (!post) return new Response('Not found', { status: 404 });
  const comment = await db.prepare('SELECT * FROM comments WHERE id = ? AND post_id = ?').bind(commentId, postId).first();
  if (!comment) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(comment));
}

export async function deletePostComment(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { postId, commentId } = req.params;
  const result = await db.prepare(`
    DELETE FROM comments WHERE id = ? AND post_id IN (SELECT id FROM posts WHERE id = ? AND user_id = ?)
  `).bind(commentId, postId, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response('Deleted');
}

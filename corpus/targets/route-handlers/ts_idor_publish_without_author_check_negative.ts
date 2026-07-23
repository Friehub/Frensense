// SAFE: Verifies authorship before publish/unpublish
export async function publishArticle(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { articleId } = req.body;
  const result = await db.prepare('UPDATE articles SET published = 1, published_at = ? WHERE id = ? AND author_id = ?').bind(Date.now(), articleId, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify({ published: true }));
}

export async function unpublishPost(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { postId } = req.params;
  const result = await db.prepare('UPDATE posts SET published = 0 WHERE id = ? AND author_id = ?').bind(postId, session.userId).run();
  if (!result) return new Response('Not found', { status: 404 });
  return new Response('Unpublished');
}

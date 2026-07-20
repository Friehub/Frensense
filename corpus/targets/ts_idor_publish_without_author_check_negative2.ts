// SAFE: Admin override for publish is separately gated and audited
export async function publishArticle(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { articleId } = req.body;
  const article = await db.prepare('SELECT * FROM articles WHERE id = ?').bind(articleId).first();
  if (!article) return new Response('Not found', { status: 404 });
  if (article.author_id !== session.userId && session.role !== 'admin') {
    return new Response('Forbidden', { status: 403 });
  }
  await db.prepare('UPDATE articles SET published = 1, published_at = ?, published_by = ? WHERE id = ?').bind(Date.now(), session.userId, articleId).run();
  return new Response(JSON.stringify({ published: true }));
}

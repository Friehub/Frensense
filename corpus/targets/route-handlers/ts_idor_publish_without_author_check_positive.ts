// [frensense]
// observation: A publish/unpublish endpoint for content (articles, posts, comments) accepts a content ID without verifying that the current user is the author.
// impact: An attacker can publish or unpublish any user's content, causing reputational damage, content manipulation, or denial of service.
// improvement: Verify that the current user is the author of the content before allowing publish/unpublish actions.

export async function publishArticle(req: Request, db: DB): Promise<Response> {
  const { articleId } = req.body;
  await db.prepare('UPDATE articles SET published = 1, published_at = ? WHERE id = ?').bind(Date.now(), articleId).run();
  return new Response(JSON.stringify({ published: true }));
}

export async function unpublishPost(req: Request, db: DB): Promise<Response> {
  const { postId } = req.params;
  await db.prepare('UPDATE posts SET published = 0 WHERE id = ?').bind(postId).run();
  return new Response('Unpublished');
}

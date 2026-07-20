// SAFE: Enforces that the content must be in APPROVED state before it can be shipped

export async function shipContent(contentId: string, env: Env) {
  const content = await env.DB.prepare(
    'SELECT status FROM content WHERE id = ?'
  ).bind(contentId).first();

  if (!content) throw new Error('Not found');

  if (content.status !== 'APPROVED') {
    throw new Error(
      `Content must be APPROVED before shipping; current status: ${content.status}`
    );
  }

  const result = await env.DB.prepare(
    'UPDATE content SET status = ? WHERE id = ? AND status = ?'
  ).bind('SHIPPED', contentId, 'APPROVED').run();

  if (result.meta.changes === 0) {
    throw new Error('Concurrent modification detected');
  }

  await notifySubscribers(contentId, env);
}

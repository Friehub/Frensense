// SAFE: Verifies premium status from the authenticated user's database record, not from URL parameters

export async function getPremiumContent(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  // SAFE: check subscription from DB, not from URL params
  const sub = await env.DB.prepare(
    'SELECT status FROM subscriptions WHERE user_id = ? AND status = ? AND end_date > ?'
  ).bind(auth.userId, 'ACTIVE', Date.now()).first();

  if (!sub) {
    return new Response('Premium subscription required', { status: 403 });
  }

  const content = await env.DB.prepare(
    'SELECT * FROM premium_content WHERE published = 1'
  ).all();

  return Response.json(content);
}

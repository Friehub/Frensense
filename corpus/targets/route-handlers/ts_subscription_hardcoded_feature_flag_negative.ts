// SAFE: Checks subscription status from the database instead of using a hardcoded flag

export async function getPremiumContent(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  // SAFE: dynamic check from database
  const entitlement = await env.DB.prepare(
    'SELECT id FROM entitlements WHERE user_id = ? AND feature = ? AND active = 1'
  ).bind(auth.userId, 'premium_content').first();

  if (!entitlement) {
    return new Response('Premium subscription required', { status: 403 });
  }

  const content = await env.DB.prepare(
    'SELECT * FROM premium_content WHERE published = 1'
  ).all();

  return Response.json(content);
}

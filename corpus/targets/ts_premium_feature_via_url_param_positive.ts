// [frensense]
// observation: A premium feature is gated by a URL query parameter that the client can set, with no server-side verification of subscription status.
// impact: Any user can access premium features by simply adding ?premium=true to the URL, bypassing the billing system entirely.
// improvement: Always verify the user's subscription or entitlement server-side from the database, never from client-provided parameters.

export async function getPremiumContent(req: Request, env: Env) {
  const url = new URL(req.url);
  const isPremium = url.searchParams.get('premium') === 'true';

  // VULNERABLE: trusts client-provided URL parameter
  if (!isPremium) {
    return new Response('Upgrade to premium', { status: 403 });
  }

  const content = await env.DB.prepare(
    'SELECT * FROM premium_content WHERE published = 1'
  ).all();

  return Response.json(content);
}

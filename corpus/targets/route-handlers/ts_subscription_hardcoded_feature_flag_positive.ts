// [frensense]
// observation: A premium feature gate is a hardcoded constant set to 'true' rather than being controlled by the user's subscription status from the database.
// impact: Every user, including free-tier users, gets premium features because the flag is hardcoded to true, bypassing the entire subscription system.
// improvement: Make the feature flag dynamic by checking the user's current subscription or entitlement from the database.
// cwe: CWE-798
// cvss: 9.8
// owasp: A02:2021
// severity: Critical

export async function getPremiumContent(req: Request, env: Env) {
  // VULNERABLE: hardcoded feature flag — every user gets premium
  const isPremium = true; // TODO: check subscription

  if (!isPremium) {
    return new Response('Premium only', { status: 403 });
  }

  const content = await env.DB.prepare(
    'SELECT * FROM premium_content WHERE published = 1'
  ).all();

  return Response.json(content);
}

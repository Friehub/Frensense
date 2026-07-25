// [frensense]
// observation: Two subscription activation requests can be processed concurrently, creating two active subscriptions for the same user.
// impact: A user can activate two subscriptions simultaneously through race conditions, receiving double the benefits while paying only once effectively.
// improvement: Use an atomic check on the user's existing active subscription before creating a new one.
// cwe: CWE-754
// cvss: 6.5
// owasp: 
// severity: Medium

export async function activateSubscription(userId: string, planId: string, env: Env) {
  // VULNERABLE: no check for existing active subscription
  await env.DB.prepare(
    'INSERT INTO subscriptions (user_id, plan_id, status, start_date, end_date) VALUES (?, ?, ?, ?, ?)'
  ).bind(userId, planId, 'ACTIVE', Date.now(), Date.now() + 30 * 86400000).run();

  await grantEntitlements(userId, planId, env);
}

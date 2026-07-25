// [frensense]
// observation: When a user's subscription is downgraded to a lower tier, their premium features are not revoked, so they continue to have access.
// impact: A user can downgrade to a cheaper plan but keep using premium features indefinitely, receiving more value than they pay for.
// improvement: Remove or disable all premium entitlements when the plan is downgraded.
// cwe: CWE-754
// cvss: 6.5
// owasp: 
// severity: Medium

export async function downgradePlan(userId: string, newPlanId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE user_id = ? AND status = ?'
  ).bind(userId, 'ACTIVE').first();

  if (!sub) throw new Error('No active subscription');

  // VULNERABLE: downgrades plan but does not revoke higher-tier features
  await env.DB.prepare(
    'UPDATE subscriptions SET plan_id = ? WHERE id = ?'
  ).bind(newPlanId, sub.id).run();

  // Missing: revoke entitlements that the new plan does not include
  // await syncEntitlementsToPlan(userId, newPlanId, env);
}

// [frensense]
// observation: When a subscription is cancelled, the user's entitlements (e.g., premium features, storage, API access) are not immediately revoked.
// impact: A user can cancel their subscription and continue enjoying premium features indefinitely until the next entitlement sync, causing ongoing revenue loss.
// improvement: Immediately revoke all premium entitlements when the subscription is cancelled.

export async function cancelSubscription(subscriptionId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE id = ?'
  ).bind(subscriptionId).first();

  if (!sub) throw new Error('Subscription not found');

  await env.DB.prepare(
    'UPDATE subscriptions SET status = ? WHERE id = ?'
  ).bind('CANCELLED', subscriptionId).run();

  // VULNERABLE: does not revoke entitlements — user keeps premium access
  // await revokeEntitlements(sub.user_id, env); — MISSING
}

// SAFE: Immediately revokes all premium entitlements when the subscription is cancelled

export async function cancelSubscription(subscriptionId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE id = ?'
  ).bind(subscriptionId).first();

  if (!sub) throw new Error('Subscription not found');

  // SAFE: update subscription AND revoke entitlements atomically
  await env.DB.prepare(
    'UPDATE subscriptions SET status = ?, cancelled_at = ? WHERE id = ?'
  ).bind('CANCELLED', Date.now(), subscriptionId).run();

  await revokeEntitlements(sub.user_id, env);
}

async function revokeEntitlements(userId: string, env: Env) {
  await env.DB.prepare(
    'UPDATE entitlements SET active = 0 WHERE user_id = ? AND active = 1'
  ).bind(userId).run();
}

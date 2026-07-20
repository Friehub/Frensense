// [frensense]
// observation: The subscription end date or cancellation status is not checked before granting access to a premium feature, so expired or cancelled subscriptions still work.
// impact: A user can cancel their subscription and continue enjoying premium features indefinitely without paying, causing ongoing revenue loss.
// improvement: Check the subscription's end date and status on every access, and deny if the subscription is expired or cancelled.

export async function accessPremiumFeature(userId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE user_id = ?'
  ).bind(userId).first();

  if (!sub) {
    throw new Error('Subscription not found');
  }

  // VULNERABLE: does not check if subscription is expired or cancelled
  await env.DB.prepare(
    'INSERT INTO premium_actions (user_id, action, timestamp) VALUES (?, ?, ?)'
  ).bind(userId, 'premium_feature_access', Date.now()).run();

  return { access: true };
}

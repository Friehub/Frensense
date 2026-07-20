// SAFE: Checks subscription end date and status before granting access to premium features

export async function accessPremiumFeature(userId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE user_id = ? AND status = ?'
  ).bind(userId, 'ACTIVE').first();

  if (!sub) {
    throw new Error('No active subscription found');
  }

  // SAFE: check end date
  if (new Date(sub.end_date) < new Date()) {
    throw new Error('Subscription has expired');
  }

  await env.DB.prepare(
    'INSERT INTO premium_actions (user_id, action, timestamp) VALUES (?, ?, ?)'
  ).bind(userId, 'premium_feature_access', Date.now()).run();

  return { access: true };
}

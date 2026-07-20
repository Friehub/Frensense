// SAFE: Checks for existing active subscription before creating a new one

export async function activateSubscription(userId: string, planId: string, env: Env) {
  const existing = await env.DB.prepare(
    'SELECT id FROM subscriptions WHERE user_id = ? AND status = ?'
  ).bind(userId, 'ACTIVE').first();

  if (existing) {
    throw new Error('User already has an active subscription');
  }

  const inserted = await env.DB.prepare(
    'INSERT INTO subscriptions (user_id, plan_id, status, start_date, end_date) VALUES (?, ?, ?, ?, ?)'
  ).bind(userId, planId, 'ACTIVE', Date.now(), Date.now() + 30 * 86400000).run();

  if (inserted.meta.changes > 0) {
    await grantEntitlements(userId, planId, env);
  }
}

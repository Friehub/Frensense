// SAFE: Revokes higher-tier features that are not included in the downgraded plan

export async function downgradePlan(userId: string, newPlanId: string, env: Env) {
  const sub = await env.DB.prepare(
    'SELECT * FROM subscriptions WHERE user_id = ? AND status = ?'
  ).bind(userId, 'ACTIVE').first();

  if (!sub) throw new Error('No active subscription');

  const newPlan = await env.DB.prepare(
    'SELECT * FROM plans WHERE id = ?'
  ).bind(newPlanId).first();

  if (!newPlan) throw new Error('Plan not found');

  // SAFE: remove features not in the new plan
  await env.DB.prepare(
    'DELETE FROM entitlements WHERE user_id = ? AND feature NOT IN (' +
    '(SELECT feature FROM plan_features WHERE plan_id = ?)' +
    ')'
  ).bind(userId, newPlanId).run();

  await env.DB.prepare(
    'UPDATE subscriptions SET plan_id = ? WHERE id = ?'
  ).bind(newPlanId, sub.id).run();
}

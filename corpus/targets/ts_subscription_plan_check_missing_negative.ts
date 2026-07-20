// SAFE: Checks that the user's plan includes the requested feature before allowing access

export async function useFeature(userId: string, featureName: string, env: Env) {
  const user = await env.DB.prepare(
    'SELECT plan_id FROM users WHERE id = ?'
  ).bind(userId).first();

  if (!user) throw new Error('User not found');

  const planFeature = await env.DB.prepare(
    'SELECT id FROM plan_features WHERE plan_id = ? AND feature = ?'
  ).bind(user.plan_id, featureName).first();

  if (!planFeature) {
    throw new Error(`Feature "${featureName}" is not available on your plan`);
  }

  await env.DB.prepare(
    'INSERT INTO feature_usage (user_id, feature, used_at) VALUES (?, ?, ?)'
  ).bind(userId, featureName, Date.now()).run();

  return executeFeature(featureName);
}

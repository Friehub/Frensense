// [frensense]
// observation: A feature that requires a specific plan is accessed without checking the user's current plan, so any user can use it.
// impact: Free-tier or lower-tier users can access features that should be restricted to higher-tier plans, bypassing the tiered pricing model.
// improvement: Check the user's plan before allowing access to plan-specific features.

export async function useFeature(userId: string, featureName: string, env: Env) {
  // VULNERABLE: no plan check — any user can use any feature
  await env.DB.prepare(
    'INSERT INTO feature_usage (user_id, feature, used_at) VALUES (?, ?, ?)'
  ).bind(userId, featureName, Date.now()).run();

  return executeFeature(featureName);
}

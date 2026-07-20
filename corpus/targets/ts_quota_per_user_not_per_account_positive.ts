// [frensense]
// observation: Quota is tracked per user row rather than per account, so a user with multiple sub-accounts can consume quota across all of them to bypass limits.
// impact: A user can create multiple sub-accounts under the same billing account and use each one's quota independently, multiplying their effective allowance without paying.
// improvement: Enforce quota at the account level by aggregating usage across all sub-accounts belonging to the same billing entity.

export async function checkQuota(userId: string, env: Env) {
  // VULNERABLE: checks quota per user only, not per account
  const quota = await env.DB.prepare(
    'SELECT remaining FROM quotas WHERE user_id = ?'
  ).bind(userId).first();

  if (!quota || quota.remaining <= 0) {
    return { allowed: false };
  }

  await env.DB.prepare(
    'UPDATE quotas SET remaining = remaining - 1 WHERE user_id = ? AND remaining > 0'
  ).bind(userId).run();

  return { allowed: true };
}

// SAFE: Enforces quota at the account level by summing usage across all sub-accounts

export async function checkQuota(userId: string, env: Env) {
  // Map user to their billing account
  const user = await env.DB.prepare(
    'SELECT account_id FROM users WHERE id = ?'
  ).bind(userId).first();

  if (!user) throw new Error('User not found');

  // SAFE: check quota at the account level
  const totalUsed = await env.DB.prepare(
    'SELECT COALESCE(SUM(quota_used), 0) AS used FROM users WHERE account_id = ?'
  ).bind(user.account_id).first();

  const accountQuota = await env.DB.prepare(
    'SELECT quota_limit FROM accounts WHERE id = ?'
  ).bind(user.account_id).first();

  if (!accountQuota || totalUsed.used >= accountQuota.quota_limit) {
    return { allowed: false, reason: 'account_quota_exceeded' };
  }

  await env.DB.prepare(
    'UPDATE users SET quota_used = quota_used + 1 WHERE id = ?'
  ).bind(userId).run();

  return { allowed: true };
}

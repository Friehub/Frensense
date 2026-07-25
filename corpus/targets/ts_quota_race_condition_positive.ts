// [frensense]
// observation: The quota check reads the current value, then subtracts in a separate write, allowing concurrent requests to both succeed and over-consume.
// impact: Two concurrent requests both read quota.remaining = 1, both pass the check, and both decrement, resulting in quota.remaining = -1.
// improvement: Use an atomic decrement operation (UPDATE ... SET remaining = remaining - 1 WHERE remaining > 0) instead of read-check-write.
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

export async function consumeQuota(userId: string, env: Env) {
  // VULNERABLE: read-check-write race condition
  const raw = await env.QUOTA_KV.get(`quota:${userId}`);
  const quota = raw ? JSON.parse(raw) : null;

  if (!quota || quota.remaining <= 0) {
    return { allowed: false };
  }

  const newRemaining = quota.remaining - 1;
  await env.QUOTA_KV.put(
    `quota:${userId}`,
    JSON.stringify({ ...quota, remaining: newRemaining })
  );

  return { allowed: true, remaining: newRemaining };
}

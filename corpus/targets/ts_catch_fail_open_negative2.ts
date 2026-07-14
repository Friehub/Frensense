// SAFE variant 2: fail closed in production when auth service is unreachable
async function checkAndConsumeQuota(userId: string, env: Env) {
  try {
    const raw = await env.QUOTA_KV.get(`quota:${userId}`);
    const quota = raw ? JSON.parse(raw) : null;
    if (!quota || quota.remaining <= 0) return { allowed: false, reason: 'quota_exceeded' };
    await env.QUOTA_KV.put(`quota:${userId}`, JSON.stringify({ ...quota, remaining: quota.remaining - 1 }));
    return { allowed: true, remaining: quota.remaining - 1 };
  } catch (e) {
    console.error('checkAndConsumeQuota failed:', e);
    // SAFE: fail closed in production; only allow in dev
    if (env.ENVIRONMENT === 'development') return { allowed: true, remaining: 999 };
    return { allowed: false, reason: 'quota_service_unavailable' };
  }
}

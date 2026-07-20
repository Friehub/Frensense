// SAFE: Fails closed — any error in the quota check denies access

export async function checkQuota(userId: string, env: Env) {
  try {
    const raw = await env.QUOTA_KV.get(`quota:${userId}`);
    const quota = raw ? JSON.parse(raw) : null;

    if (!quota || quota.remaining <= 0) {
      return { allowed: false, reason: 'quota_exceeded' };
    }

    await env.QUOTA_KV.put(
      `quota:${userId}`,
      JSON.stringify({ ...quota, remaining: quota.remaining - 1 })
    );

    return { allowed: true, remaining: quota.remaining - 1 };
  } catch (e) {
    // SAFE: fail closed — deny access when quota service fails
    console.error('Quota check failed:', e);
    return { allowed: false, reason: 'quota_service_error' };
  }
}

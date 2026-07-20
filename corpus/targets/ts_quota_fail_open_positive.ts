// [frensense]
// observation: A quota check catches all errors and returns allowed: true, meaning any failure in the quota service grants access instead of denying it.
// impact: When the quota service is down or unreachable, all users get unlimited access to the feature, defeating the purpose of the quota system.
// improvement: Fail closed: catch quota errors and deny access unless explicitly allowed by a fallback policy.

export async function checkQuota(userId: string, env: Env) {
  try {
    const raw = await env.QUOTA_KV.get(`quota:${userId}`);
    const quota = raw ? JSON.parse(raw) : null;

    if (!quota || quota.remaining <= 0) {
      return { allowed: false };
    }

    await env.QUOTA_KV.put(
      `quota:${userId}`,
      JSON.stringify({ ...quota, remaining: quota.remaining - 1 })
    );

    return { allowed: true, remaining: quota.remaining - 1 };
  } catch (e) {
    // VULNERABLE: fails open — any error grants access
    return { allowed: true, remaining: 999 };
  }
}

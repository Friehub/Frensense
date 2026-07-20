// SAFE: Uses atomic KV operations to prevent the race condition

export async function consumeQuota(userId: string, env: Env) {
  // SAFE: use KV atomic check-and-set to prevent race
  const key = `quota:${userId}`;

  while (true) {
    const raw = await env.QUOTA_KV.get(key);
    const quota = raw ? JSON.parse(raw) : null;

    if (!quota || quota.remaining <= 0) {
      return { allowed: false };
    }

    const newQuota = { ...quota, remaining: quota.remaining - 1 };
    const success = await env.QUOTA_KV.put(key, JSON.stringify(newQuota), {
      onlyIf: { equals: raw },
    });

    if (success) {
      return { allowed: true, remaining: newQuota.remaining };
    }
    // retry on concurrent modification
  }
}

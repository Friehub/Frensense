// SAFE: Enforces quota on every execution attempt, including retries

export async function processWithRetry(userId: string, input: string, env: Env) {
  let lastError: Error | null = null;

  for (let attempt = 0; attempt < 3; attempt++) {
    try {
      // SAFE: quota checked on every attempt
      const quotaResult = await checkQuota(userId, env);
      if (!quotaResult.allowed) throw new Error('Quota exceeded');

      return await env.AI.run('@cf/meta/llama-3-8b-instruct', {
        messages: [{ role: 'user', content: input }],
      });
    } catch (e) {
      lastError = e as Error;
      await delay(1000 * (attempt + 1));
    }
  }

  throw lastError;
}

async function checkQuota(userId: string, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE quotas SET remaining = remaining - 1 WHERE user_id = ? AND remaining > 0'
  ).bind(userId).run();

  return { allowed: result.meta.changes > 0 };
}

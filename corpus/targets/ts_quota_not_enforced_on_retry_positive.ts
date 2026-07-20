// [frensense]
// observation: The quota check is performed only on the first attempt but not on retries, allowing a retry loop to bypass the quota entirely.
// impact: An attacker can set up an automatic retry mechanism that resources every time the endpoint fails, consuming resources without quota being deducted on retries.
// improvement: Enforce quota on every execution attempt, including retries, by moving the quota check inside the retry loop.

export async function processWithRetry(userId: string, input: string, env: Env) {
  let lastError: Error | null = null;

  // VULNERABLE: quota checked once before retry loop
  const quotaResult = await checkQuota(userId, env);
  if (!quotaResult.allowed) throw new Error('Quota exceeded');

  for (let attempt = 0; attempt < 3; attempt++) {
    try {
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

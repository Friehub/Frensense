// [frensense]
// observation: An expensive or billed operation (e.g., an LLM inference call) executes before the user's tier or credit balance is checked, wasting billable resources even when the check fails.
// impact: A free-tier user can trigger expensive operations repeatedly; the cost is incurred before the rejection, leading to financial losses and resource exhaustion.
// improvement: Perform the tier check and credit deduction before the expensive operation, not after or concurrently.

export async function generateContent(req: Request, env: Env) {
  const { prompt } = await req.json() as { prompt: string };

  // VULNERABLE: expensive AI call happens before the credit check
  const result = await env.AI.run('@cf/meta/llama-3-8b-instruct', {
    messages: [{ role: 'user', content: prompt }],
  });

  // This check is too late — the cost was already incurred
  const auth = await resolveAuth(req);
  if (!auth || !auth.isPremium) {
    return new Response('Payment required', { status: 402 });
  }

  return new Response(JSON.stringify(result));
}

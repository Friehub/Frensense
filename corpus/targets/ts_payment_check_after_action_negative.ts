// SAFE: Tier check and credit deduction are performed before the expensive operation

export async function generateContent(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });
  if (!auth.isPremium) return new Response('Payment required', { status: 402 });

  const hasCredits = await deductCredits(env, auth.customerId, 1);
  if (!hasCredits) return new Response('Insufficient credits', { status: 402 });

  // SAFE: expensive call only reached after payment gate passes
  const result = await env.AI.run('@cf/meta/llama-3-8b-instruct', {
    messages: [{ role: 'user', content: prompt }],
  });

  return new Response(JSON.stringify(result));
}

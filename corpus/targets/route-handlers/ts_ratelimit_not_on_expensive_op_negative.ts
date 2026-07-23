// SAFE: Rate limits the expensive operation using a per-user quota system

export async function generateImage(req: Request, env: Env) {
  const { prompt } = await req.json() as { prompt: string };
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  // SAFE: check and consume quota
  const key = `quota:generation:${auth.userId}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 20) {
    return new Response('Rate limit exceeded. Upgrade your plan for more.', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 3600 });

  const result = await env.AI.run('@cf/stabilityai/stable-diffusion-xl-base-1.0', {
    prompt,
  });

  return new Response(result.image);
}

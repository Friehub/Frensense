// [frensense]
// observation: An expensive operation (LLM inference, image generation, or heavy computation) has no rate limit, allowing unlimited calls that accumulate cloud costs.
// impact: A user can call the expensive endpoint thousands of times per minute, causing unexpectedly high cloud bills and potentially exhausting the budget.
// improvement: Apply a per-user rate limit on all expensive operations to control cost exposure.

export async function generateImage(req: Request, env: Env) {
  const { prompt } = await req.json() as { prompt: string };

  // VULNERABLE: no rate limit on expensive image generation
  const result = await env.AI.run('@cf/stabilityai/stable-diffusion-xl-base-1.0', {
    prompt,
  });

  return new Response(result.image);
}

export async function handleExpensiveOperation(request: Request, env: Env): Promise<Response> {
  const auth = await resolveAuth(request);
  if (!auth) return new Response("Unauthorized", { status: 401 });

  // Tier check and credit deduction
  if (!PRO_TIERS.includes(auth.tier)) {
    return new Response("Payment required", { status: 402 });
  }
  
  const hasCredits = await deductCredits(env, auth.customerId, 5);
  if (!hasCredits) {
    return new Response("Insufficient credits", { status: 402 });
  }

  const result = await env.AI.run("@cf/meta/llama-3-8b-instruct", {
    messages: [{ role: "user", content: "Do some expensive computation" }]
  });

  return new Response(JSON.stringify(result));
}

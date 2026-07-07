export async function handleExpensiveOperation(request: Request, env: Env): Promise<Response> {
  const auth = await resolveAuth(request);
  if (!auth) return new Response("Unauthorized", { status: 401 });

  // Missing tier check and credit deduction!
  // Any user can call this expensive API infinitely
  
  const result = await env.AI.run("@cf/meta/llama-3-8b-instruct", {
    messages: [{ role: "user", content: "Do some expensive computation" }]
  });

  return new Response(JSON.stringify(result));
}

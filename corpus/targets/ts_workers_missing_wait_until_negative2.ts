// SAFE alternative: single waitUntil with combined promise
export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);
    const data = await env.KV.get(url.pathname);

    ctx.waitUntil(Promise.all([
      logRequest(request),
      warmCache(url.pathname, data),
    ]));

    return new Response(data, { headers: { 'Content-Type': 'text/html' } });
  }
};

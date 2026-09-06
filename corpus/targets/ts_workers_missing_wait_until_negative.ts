// SAFE: wrap background work in ctx.waitUntil()
export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);
    const data = await env.KV.get(url.pathname);

    ctx.waitUntil(logRequest(request));
    ctx.waitUntil(warmCache(url.pathname, data));

    return new Response(data, { headers: { 'Content-Type': 'text/html' } });
  }
};

// [frensense]
// observation: Cloudflare Worker handler starts asynchronous work (logging, analytics, cache warm) without wrapping it in ctx.waitUntil().
// impact: When the handler returns its Response, the Worker runtime may terminate the event loop, killing any in-flight async operations. Analytics events are lost, cache entries go unwarmed, side effects silently disappear.
// improvement: Wrap all background async work in ctx.waitUntil() to extend the worker lifetime.

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);
    const data = await env.KV.get(url.pathname);

    // VULNERABLE: background tasks not wrapped — may be killed
    logRequest(request);
    warmCache(url.pathname, data);

    return new Response(data, { headers: { 'Content-Type': 'text/html' } });
  }
};

async function logRequest(req: Request): Promise<void> {
  await fetch('https://analytics.example.com/log', {
    method: 'POST',
    body: JSON.stringify({ url: req.url, method: req.method }),
  });
}

async function warmCache(path: string, data: string | null): Promise<void> {
  if (data) await env.KV.put(`cache:${path}`, data, { expirationTtl: 60 });
}

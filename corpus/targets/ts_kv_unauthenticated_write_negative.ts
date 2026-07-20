// SAFE: KV writes are guarded by a session-based authentication check

interface Env {
  KV: KVNamespace;
  AUTH_SECRET: string;
}

async function authenticate(request: Request, env: Env): Promise<string | null> {
  const cookie = request.headers.get('Cookie');
  const session = cookie?.match(/session=([^;]+)/)?.[1];
  if (!session) return null;
  const user = await env.KV.get(`session:${session}`);
  return user;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  const user = await authenticate(context.request, context.env);
  if (!user) return new Response('Unauthorized', { status: 401 });

  const body = await context.request.json() as { key: string, value: string };
  await context.env.KV.put(body.key, body.value);
  return new Response('OK');
};

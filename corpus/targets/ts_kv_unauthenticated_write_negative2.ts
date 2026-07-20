// SAFE: KV writes are validated against the authenticated user's namespace prefix

interface Env {
  KV: KVNamespace;
}

async function getUserPrefix(request: Request): Promise<string | null> {
  const token = request.headers.get('Authorization')?.replace('Bearer ', '');
  if (!token) return null;
  const session = await validateToken(token);
  return session ? `user:${session.userId}` : null;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  const prefix = await getUserPrefix(context.request);
  if (!prefix) return new Response('Unauthorized', { status: 401 });

  const body = await context.request.json() as { key: string, value: string };
  await context.env.KV.put(`${prefix}:${body.key}`, body.value);
  return new Response('OK');
};

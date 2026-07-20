// [frensense]
// observation: A Cloudflare Workers handler writes to KV without checking whether the user is authenticated or authorized.
// impact: Any unauthenticated user can write arbitrary data to the KV store, corrupting state or overwriting sensitive values.
// improvement: Add an authentication check before every KV.put() or KV.delete() call.

interface Env {
  KV: KVNamespace;
}

export const onRequest: PagesFunction<Env> = async (context) => {
  const body = await context.request.json() as { key: string, value: string };
  await context.env.KV.put(body.key, body.value);
  return new Response('OK');
};

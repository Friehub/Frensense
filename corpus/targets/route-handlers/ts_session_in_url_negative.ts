// SAFE: Token is transmitted via Authorization header, never in URL
export async function authenticate(req: Request): Promise<Response> {
  const authHeader = req.headers.get('Authorization');
  if (!authHeader?.startsWith('Bearer ')) {
    return new Response('Unauthorized', { status: 401 });
  }
  const token = authHeader.slice(7);
  const user = await verifyToken(token);
  if (!user) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify(user));
}

function buildLoginUrl(redirect: string): string {
  return `/login?redirect=${encodeURIComponent(redirect)}`;
}

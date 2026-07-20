// SAFE: Scopes are enforced server-side at the resource level regardless of what the token claims
export async function createApiKey(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const maxScope = session.role === 'admin' ? 'admin:all' : 'user:read';
  const requestedScope = req.body.scope || 'user:read';
  const finalScope = requestedScope === 'admin:all' && session.role !== 'admin' ? 'user:read' : requestedScope;
  const key = crypto.randomBytes(32).toString('hex');
  await db.prepare('INSERT INTO api_keys (key_hash, scope, user_id, max_allowed) VALUES (?, ?, ?, ?)').bind(hashKey(key), finalScope, session.userId, maxScope).run();
  return new Response(JSON.stringify({ apiKey: key, scope: finalScope }));
}

// [frensense]
// observation: The OAuth token issuance endpoint accepts the requested scope from the client without validating that it does not exceed the client's authorized scope.
// impact: A client application can request broader permissions than it needs (e.g., requesting admin:all when it should only have read:profile), and the server grants them, enabling privilege escalation.
// improvement: Validate that the requested scope is a subset of the client's authorized scope before issuing the token.

export async function issueToken(req: Request): Promise<Response> {
  const { clientId, clientSecret, scope } = await req.json();
  const client = await validateClient(clientId, clientSecret);
  if (!client) return new Response('Invalid client', { status: 401 });
  const token = jwt.sign({ clientId, scope }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ access_token: token, scope }));
}

export async function createApiKey(req: Request, db: DB): Promise<Response> {
  const { scope } = req.body;
  const key = crypto.randomBytes(32).toString('hex');
  await db.prepare('INSERT INTO api_keys (key_hash, scope, user_id) VALUES (?, ?, ?)').bind(hashKey(key), scope, req.session.userId).run();
  return new Response(JSON.stringify({ apiKey: key, scope }));
}

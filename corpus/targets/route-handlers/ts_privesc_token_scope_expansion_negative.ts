// SAFE: Requested scope is validated against the client's allowed scope before token issuance
const ALLOWED_CLIENT_SCOPES: Record<string, string[]> = {
  'web-app': ['openid', 'profile', 'email'],
  'mobile-app': ['openid', 'profile'],
  'admin-panel': ['openid', 'profile', 'admin:read'],
};

export async function issueToken(req: Request): Promise<Response> {
  const { clientId, clientSecret, scope } = await req.json();
  const client = await validateClient(clientId, clientSecret);
  if (!client) return new Response('Invalid client', { status: 401 });
  const allowedScopes = ALLOWED_CLIENT_SCOPES[clientId] || [];
  const requestedScopes = scope.split(' ');
  const validScopes = requestedScopes.filter(s => allowedScopes.includes(s));
  if (validScopes.length === 0) return new Response('No valid scopes requested', { status: 400 });
  const token = jwt.sign({ clientId, scope: validScopes.join(' ') }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ access_token: token, scope: validScopes.join(' ') }));
}

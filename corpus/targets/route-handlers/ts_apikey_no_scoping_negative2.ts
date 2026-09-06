// SAFE: Uses API key with JWT-based scopes stored in database
export async function authenticateApiKey(req: Request, db: DB): Promise<boolean> {
  const key = req.headers.get('x-api-key');
  const apiKey = await db.prepare('SELECT * FROM api_keys WHERE key_hash = ? AND revoked = 0').bind(hashKey(key)).first();
  if (!apiKey) return false;
  req.scope = apiKey.scope;
  req.rateLimit = apiKey.rate_limit;
  return true;
}

function hashKey(key: string): string {
  return crypto.createHash('sha256').update(key).digest('hex');
}

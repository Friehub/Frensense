// SAFE: Throws an exception instead of returning null, ensuring callers must catch or propagate
interface AuthResult { id: number; name: string; role: string; }
interface TokenPayload { sub: string; name: string; exp: number; iss: string; }

function authenticateUser(token: string, secret: string): AuthResult {
  if (!token || token.length === 0) throw new Error('Missing token');
  let decoded: TokenPayload;
  try { decoded = decodeJwt(token, secret); }
  catch (e) { throw new Error('Invalid token'); }
  if (!decoded.exp || decoded.exp < Date.now()) throw new Error('Token expired');
  if (decoded.iss !== 'auth.example.com') throw new Error('Unknown issuer');
  return { id: parseInt(decoded.sub), name: decoded.name, role: 'user' };
}

// Caller catches the error
function handler(token: string, secret: string) {
  try { const user = authenticateUser(token, secret); return user; }
  catch (e) { return null; }
}

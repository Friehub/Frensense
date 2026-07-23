// SAFE: CSPRNG-generated token stored server-side
import { randomBytes } from 'node:crypto';

function generateCsrfToken(): string {
  return randomBytes(32).toString('hex');
}

function verifyCsrfToken(token: string, sessionTokens: Set<string>): boolean {
  // SAFE: token must exist in server-side store
  return sessionTokens.has(token);
}

app.get('/api/csrf-token', (req, res) => {
  const token = generateCsrfToken();
  req.session.csrfTokens = req.session.csrfTokens || new Set();
  req.session.csrfTokens.add(token);
  res.json({ token });
});

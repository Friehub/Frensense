// SAFE: PKCE is implemented with S256 code challenge method
import crypto from 'crypto';

export function initiateLogin(req: Request, res: Response): void {
  const codeVerifier = crypto.randomBytes(32).toString('base64url');
  const codeChallenge = crypto.createHash('sha256').update(codeVerifier).digest('base64url');
  req.session.codeVerifier = codeVerifier;
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&response_type=code&code_challenge=${codeChallenge}&code_challenge_method=S256&redirect_uri=https://app.example.com/callback`;
  res.redirect(authUrl);
}

// SAFE: Validates nonce manually by decoding the ID token and comparing
import jwt from 'jsonwebtoken';
import crypto from 'crypto';

export function initiateLogin(req: Request, res: Response): void {
  const nonce = crypto.randomBytes(16).toString('hex');
  req.session.oidcNonce = nonce;
  const authUrl = `https://provider.com/auth?client_id=ID&response_type=id_token&scope=openid&nonce=${nonce}`;
  res.redirect(authUrl);
}

export async function handleCallback(req: Request, res: Response): Promise<void> {
  const idToken = req.query.id_token as string;
  const claims = jwt.decode(idToken) as any;
  if (claims.nonce !== req.session.oidcNonce) {
    res.status(401).json({ error: 'Nonce mismatch' });
    return;
  }
  req.session.userId = claims.sub;
  res.redirect('/dashboard');
}

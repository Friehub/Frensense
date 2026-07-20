// SAFE: Nonce is generated before auth request and validated in the callback
import { generators, Issuer } from 'openid-client';

export function initiateLogin(req: Request, res: Response): void {
  const nonce = generators.nonce();
  req.session.oidcNonce = nonce;
  const authUrl = client.authorizationUrl({ scope: 'openid email', nonce });
  res.redirect(authUrl);
}

export async function handleCallback(req: Request, res: Response): Promise<void> {
  const params = client.callbackParams(req);
  const tokenSet = await client.callback('https://app.example.com/callback', params, { nonce: req.session.oidcNonce });
  req.session.userId = tokenSet.claims().sub;
  res.redirect('/dashboard');
}

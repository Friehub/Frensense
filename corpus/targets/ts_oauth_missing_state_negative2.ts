// SAFE: Uses openid-client library which handles state and nonce automatically
import { generators, Client } from 'openid-client';

const client = new Client({ client_id: 'CLIENT_ID', token_endpoint_auth_method: 'none' });

export function initiateLogin(req: Request, res: Response): void {
  const state = generators.state();
  const nonce = generators.nonce();
  req.session.oauthState = state;
  req.session.oauthNonce = nonce;
  const authUrl = client.authorizationUrl({ scope: 'openid email', state, nonce });
  res.redirect(authUrl);
}

export async function handleCallback(req: Request, res: Response): Promise<void> {
  const params = client.callbackParams(req);
  const tokenSet = await client.callback('https://app.example.com/callback', params, {
    state: req.session.oauthState,
    nonce: req.session.oauthNonce
  });
  req.session.userId = tokenSet.claims().sub;
  res.redirect('/dashboard');
}

// SAFE: Uses openid-client library which handles PKCE automatically
import { generators, Client } from 'openid-client';

const client = new Client({ client_id: 'ID', token_endpoint_auth_method: 'none' });

export async function initiateLogin(req: Request, res: Response): Promise<void> {
  const codeVerifier = generators.codeVerifier();
  const codeChallenge = generators.codeChallenge(codeVerifier);
  req.session.codeVerifier = codeVerifier;
  const authUrl = client.authorizationUrl({
    scope: 'openid email',
    code_challenge: codeChallenge,
    code_challenge_method: 'S256'
  });
  res.redirect(authUrl);
}

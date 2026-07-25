// [frensense]
// observation: The OIDC callback verifies the ID token signature and expiration but does not validate the nonce claim.
// impact: An attacker can replay an intercepted ID token in a different authentication session, potentially linking a victim's account to an attacker-controlled identity.
// improvement: Generate a nonce before the authorization request, store it in the session, and validate it against the nonce claim in the ID token.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Issuer } from 'openid-client';

export async function handleCallback(req: Request, res: Response): Promise<void> {
  const client = await getClient();
  const params = client.callbackParams(req);
  const tokenSet = await client.callback('https://app.example.com/callback', params);
  req.session.userId = tokenSet.claims().sub;
  res.redirect('/dashboard');
}

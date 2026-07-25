// [frensense]
// observation: A public OAuth client (no client secret) uses the authorization code flow without PKCE, meaning the authorization code can be exchanged without proving possession of the original request.
// impact: An attacker who intercepts the authorization code (e.g., through a man-in-the-middle or malicious app on the device) can exchange it for tokens without needing the client secret.
// improvement: Always use PKCE (Proof Key for Code Exchange) with public clients. Generate a code_verifier and code_challenge (S256) before initiating the auth request.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

export function initiateLogin(req: Request, res: Response): void {
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&response_type=code&redirect_uri=https://app.example.com/callback`;
  res.redirect(authUrl);
}

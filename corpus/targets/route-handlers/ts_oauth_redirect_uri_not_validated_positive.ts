// [frensense]
// observation: The OAuth callback uses a redirect_uri parameter directly from the request without validating it against a whitelist of allowed URIs.
// impact: An attacker can use the OAuth flow as an open redirect to exfiltrate authorization codes via the query string to an attacker-controlled server.
// improvement: Maintain a strict whitelist of allowed redirect URIs and validate the incoming redirect_uri against it.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

export function initiateOAuth(req: Request, res: Response): void {
  const redirectUri = req.query.redirect_uri as string;
  const authUrl = `https://provider.com/oauth?client_id=ID&redirect_uri=${redirectUri}&response_type=code`;
  res.redirect(authUrl);
}

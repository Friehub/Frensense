// [frensense]
// observation: The OAuth authorization flow initiates without generating or validating the state parameter, leaving the callback vulnerable to CSRF attacks.
// impact: An attacker can initiate an OAuth flow, intercept the callback, and link the victim's account to an attacker-controlled identity, resulting in account takeover.
// improvement: Generate a cryptographically random state value, store it in the session before redirect, and verify it in the callback handler.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

export function initiateGoogleLogin(req: Request, res: Response): void {
  const redirectUri = 'https://app.example.com/callback';
  const authUrl = `https://accounts.google.com/o/oauth2/auth?client_id=CLIENT_ID&redirect_uri=${redirectUri}&response_type=code&scope=openid%20email`;
  res.redirect(authUrl);
}

export async function handleCallback(req: Request, res: Response, db: DB): Promise<void> {
  const { code } = req.query;
  const tokens = await exchangeCode(code as string);
  const user = await db.prepare('SELECT * FROM users WHERE google_id = ?').bind(tokens.sub).first();
  if (user) {
    req.session.userId = user.id;
    res.redirect('/dashboard');
  } else {
    res.redirect('/register');
  }
}

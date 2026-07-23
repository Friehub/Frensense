// SAFE: Redirect URI is validated against a whitelist before forwarding to OAuth provider
const ALLOWED_REDIRECTS = [
  'https://app.example.com/callback',
  'https://app.example.com/auth/callback',
  'http://localhost:3000/callback'
];

export function initiateOAuth(req: Request, res: Response): void {
  const redirectUri = req.query.redirect_uri as string;
  if (!ALLOWED_REDIRECTS.includes(redirectUri)) {
    res.status(400).json({ error: 'Invalid redirect URI' });
    return;
  }
  const authUrl = `https://provider.com/oauth?client_id=ID&redirect_uri=${encodeURIComponent(redirectUri)}&response_type=code`;
  res.redirect(authUrl);
}

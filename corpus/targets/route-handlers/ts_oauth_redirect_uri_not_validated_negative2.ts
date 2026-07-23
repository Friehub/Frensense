// SAFE: Redirect URI validated with URL parsing plus prefix matching against allowed base URLs
const ALLOWED_BASES = ['https://app.example.com', 'https://app.frensense.io'];

export function initiateOAuth(req: Request, res: Response): void {
  const redirectUri = req.query.redirect_uri as string;
  try {
    const parsed = new URL(redirectUri);
    if (!ALLOWED_BASES.some(base => parsed.origin === base && parsed.pathname.startsWith('/oauth/callback'))) {
      throw new Error('Invalid redirect');
    }
  } catch {
    res.status(400).json({ error: 'Invalid redirect URI' });
    return;
  }
  const authUrl = `https://provider.com/oauth?client_id=ID&redirect_uri=${encodeURIComponent(redirectUri)}&response_type=code`;
  res.redirect(authUrl);
}

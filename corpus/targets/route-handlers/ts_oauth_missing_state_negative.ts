// SAFE: State parameter is generated, stored in session, and verified on callback
import crypto from 'crypto';

export function initiateGoogleLogin(req: Request, res: Response): void {
  const state = crypto.randomBytes(32).toString('hex');
  req.session.oauthState = state;
  const redirectUri = 'https://app.example.com/callback';
  const authUrl = `https://accounts.google.com/o/oauth2/auth?client_id=CLIENT_ID&redirect_uri=${redirectUri}&response_type=code&scope=openid%20email&state=${state}`;
  res.redirect(authUrl);
}

export async function handleCallback(req: Request, res: Response, db: DB): Promise<void> {
  const { code, state } = req.query;
  if (state !== req.session.oauthState) {
    res.status(401).json({ error: 'Invalid state parameter' });
    return;
  }
  delete req.session.oauthState;
  const tokens = await exchangeCode(code as string);
  const user = await db.prepare('SELECT * FROM users WHERE google_id = ?').bind(tokens.sub).first();
  if (user) {
    req.session.userId = user.id;
    res.redirect('/dashboard');
  } else {
    res.redirect('/register');
  }
}

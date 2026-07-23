// SAFE: Scopes are defined as a restricted constant and requested using incremental authorization
const REQUIRED_SCOPES = ['openid', 'email'];

export function initiateLogin(req: Request, res: Response): void {
  const scope = REQUIRED_SCOPES.join(' ');
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&scope=${encodeURIComponent(scope)}&response_type=code`;
  res.redirect(authUrl);
}

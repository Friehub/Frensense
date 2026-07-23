// SAFE: Only requests the minimum scopes required for the application's functionality
export function initiateLogin(req: Request, res: Response): void {
  const authUrl = `https://provider.com/oauth/authorize?client_id=ID&scope=openid%20email&response_type=code`;
  res.redirect(authUrl);
}

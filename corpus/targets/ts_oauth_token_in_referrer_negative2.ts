// SAFE: Uses form_post response mode — token delivered via POST, never in URL
// Initiation remains the same; provider configured to POST tokens to callback endpoint
export function handleCallback(req: Request, res: Response): void {
  const token = req.body.access_token;
  req.session.token = token;
  res.redirect('/dashboard');
}

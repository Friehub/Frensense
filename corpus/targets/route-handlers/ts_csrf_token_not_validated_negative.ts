// SAFE: validate CSRF token server-side
function validateCsrfToken(req: Request, res: Response, next: NextFunction) {
  const token = req.headers['x-csrf-token'];
  if (!token || token !== req.session.csrfToken) {
    return res.status(403).json({ error: 'Invalid CSRF token' });
  }
  next();
}

app.post('/api/transfer', validateCsrfToken, (req, res) => {
  performTransfer(req.body);
  res.json({ status: 'ok' });
});

app.post('/api/update-profile', validateCsrfToken, (req, res) => {
  updateProfile(req.user.id, req.body);
  res.json({ status: 'ok' });
});

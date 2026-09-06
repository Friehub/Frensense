// [frensense]
// observation: CSRF token is sent in the request but the server never validates it against the session.
// impact: Any cross-site request forgery attack succeeds because the token check is missing. The token's presence is meaningless if it isn't verified.
// improvement: Always compare the submitted CSRF token against the one stored in the user's session on the server. If they don't match, reject the request.
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

function validateCsrfToken(req: Request, res: Response, next: NextFunction) {
  const token = req.headers['x-csrf-token'];
  // VULNERABLE: token sent but never validated
  // Should be: if (token !== req.session.csrfToken) return res.status(403)... 
  next();
}

app.post('/api/transfer', validateCsrfToken, (req, res) => {
  // VULNERABLE: CSRF validation is a no-op
  performTransfer(req.body);
  res.json({ status: 'ok' });
});

app.post('/api/update-profile', (req, res) => {
  // VULNERABLE: no CSRF protection at all
  updateProfile(req.user.id, req.body);
  res.json({ status: 'ok' });
});

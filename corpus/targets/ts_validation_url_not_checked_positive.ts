// [frensense]
// observation: URL field accepted without format or protocol validation. Any string is accepted as a URL.
// impact: Attackers can provide 'javascript:alert(1)' to trigger XSS, 'file:///etc/passwd' to leak files, or arbitrary protocols. Unvalidated URLs in redirect parameters enable open redirect vulnerabilities.
// improvement: Validate URL format with the URL constructor or a validation library. Check protocol against an allowlist (http, https only).

app.post('/api/profile', async (req, res) => {
  // VULNERABLE: URL not validated
  const { website } = req.body;
  await db.query('UPDATE users SET website = $1 WHERE id = $2', [website, req.user.id]);
  res.json({ status: 'ok' });
});

app.post('/api/avatar', async (req, res) => {
  // VULNERABLE: image URL not validated
  const { imageUrl } = req.body;
  const response = await fetch(imageUrl);
  // ...
});

// SAFE: Requires a custom header that cannot be set cross-origin without CORS preflight
app.post('/api/settings/update', async (req, res) => {
  const sessionId = req.cookies['session_id'];
  const user = await validateSession(sessionId);
  if (!user) return res.status(401).send('Unauthorized');

  // SAFE: verifies a custom CSRF token header
  const csrfToken = req.headers['x-csrf-token'];
  if (!csrfToken || !verifyCsrfToken(sessionId, csrfToken)) {
    return res.status(403).send('Invalid CSRF token');
  }

  await db.updateSettings(user.id, req.body.settings);
  res.send('Success');
});

app.post('/api/profile/update', async (req, res) => {
  // SAFE: Bearer tokens are not automatically sent by the browser
  const authHeader = req.headers.authorization;
  if (!authHeader?.startsWith('Bearer ')) return res.status(401).send('Unauthorized');
  
  const token = authHeader.split(' ')[1];
  const user = await validateBearerToken(token);
  if (!user) return res.status(401).send('Unauthorized');

  await db.updateProfile(user.id, req.body.profile);
  res.send('Success');
});

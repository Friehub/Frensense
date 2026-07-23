// [frensense]
// observation: OAuth access token from an identity provider (Google, GitHub, Facebook) is accepted without server-side introspection or validation.
// impact: An attacker can forge tokens, reuse expired tokens, or use tokens from other users. Without validation against the provider's introspection endpoint, any string that passes client-side checks is accepted.
// improvement: Validate the token server-side using the provider's token introspection endpoint or by verifying the JWT signature and claims.

app.post('/api/auth/google', async (req, res) => {
  // VULNERABLE: token not validated server-side
  const { accessToken } = req.body;

  // Should call Google's tokeninfo endpoint
  const userInfo = await fetch('https://www.googleapis.com/oauth2/v2/userinfo', {
    headers: { Authorization: `Bearer ${accessToken}` },
  }).then(r => r.json());

  const user = await db.findOrCreateUser({ email: userInfo.email, name: userInfo.name });
  const token = signToken({ id: user.id });
  res.json({ token });
});

app.post('/api/auth/github', async (req, res) => {
  const { accessToken } = req.body;
  // VULNERABLE: no token validation
  const userInfo = await fetch('https://api.github.com/user', {
    headers: { Authorization: `Bearer ${accessToken}` },
  }).then(r => r.json());

  const user = await db.findOrCreateUser({ githubId: userInfo.id, name: userInfo.login });
  const token = signToken({ id: user.id });
  res.json({ token });
});

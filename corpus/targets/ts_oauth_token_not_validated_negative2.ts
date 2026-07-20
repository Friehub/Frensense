// SAFE alternative: introspect token against provider
app.post('/api/auth/google', async (req, res) => {
  const { accessToken } = req.body;

  // SAFE: call introspection endpoint
  const introspection = await fetch(`https://oauth2.googleapis.com/tokeninfo?access_token=${accessToken}`)
    .then(r => r.json());

  if (introspection.error || introspection.aud !== process.env.GOOGLE_CLIENT_ID) {
    return res.status(401).json({ error: 'Invalid token' });
  }

  const user = await db.findOrCreateUser({ email: introspection.email, name: introspection.name });
  const token = signToken({ id: user.id });
  res.json({ token });
});

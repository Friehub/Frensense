// SAFE: generic error message for all auth failures
app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    return res.status(401).json({ error: 'Invalid email or password' });
  }

  const token = signToken({ id: user.id });
  res.json({ token });
});

app.post('/api/reset-password', async (req, res) => {
  const { email } = req.body;
  const user = await db.findUserByEmail(email);
  // SAFE: same response regardless of whether user exists
  if (user) await sendResetEmail(user.email);
  res.json({ message: 'If that email is registered, a reset link has been sent' });
});

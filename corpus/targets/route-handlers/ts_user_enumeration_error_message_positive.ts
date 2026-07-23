// [frensense]
// observation: Login endpoint returns different error messages for 'user not found' vs 'wrong password', allowing attackers to enumerate valid usernames.
// impact: An attacker can compile a list of valid user accounts by trying many emails/usernames and observing the error message. This list feeds credential stuffing, phishing, or targeted attacks.
// improvement: Return the same generic error message regardless of which credential is incorrect.

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user) {
    // VULNERABLE: reveals user existence
    return res.status(401).json({ error: 'User not found' });
  }

  const valid = await bcrypt.compare(password, user.passwordHash);
  if (!valid) {
    return res.status(401).json({ error: 'Invalid password' });
  }

  const token = signToken({ id: user.id });
  res.json({ token });
});

app.post('/api/reset-password', async (req, res) => {
  const { email } = req.body;
  const user = await db.findUserByEmail(email);
  if (!user) {
    // VULNERABLE: reveals whether email is registered
    return res.json({ error: 'No account with that email' });
  }
  await sendResetEmail(user.email);
  res.json({ message: 'Reset link sent' });
});

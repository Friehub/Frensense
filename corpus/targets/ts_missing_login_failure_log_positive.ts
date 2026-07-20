// [frensense]
// observation: Failed login attempts are not logged, making brute-force attacks and account enumeration invisible.
// impact: Without login failure logs, security teams cannot detect brute-force attacks, credential stuffing, or targeted password guessing. Incident response has no data to investigate account compromises.
// improvement: Log every failed login attempt with username, IP address, timestamp, and reason for failure.

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    // VULNERABLE: failure not logged
    return res.status(401).json({ error: 'Invalid email or password' });
  }

  const token = signToken({ id: user.id });
  res.json({ token });
});

// [frensense]
// observation: Repeated authentication failures or suspicious actions do not trigger any alerting or notification.
// impact: Brute-force attacks and credential stuffing campaigns go unnoticed for days or weeks. By the time they're detected, the attacker has already compromised accounts. No automated incident response possible.
// improvement: Set up monitoring alerts for repeated failures: N failed logins within M minutes from the same IP or for the same user should trigger an alert (email, Slack, PagerDuty).

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    // VULNERABLE: repeated failures trigger no alert
    await db.query(
      'INSERT INTO login_attempts (email, ip, success, created_at) VALUES ($1, $2, false, NOW())',
      [email, req.ip]
    );
    return res.status(401).json({ error: 'Invalid credentials' });
  }
  // success
  res.json({ token: signToken({ id: user.id }) });
});

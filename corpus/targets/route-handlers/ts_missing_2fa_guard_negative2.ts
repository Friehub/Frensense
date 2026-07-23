// SAFE: Requires TOTP verification for sensitive actions using a reusable MFA middleware
function requireMfa(): express.Handler {
  return async (req, res, next) => {
    if (req.session.mfaVerified && Date.now() - req.session.mfaVerifiedAt < 15 * 60 * 1000) return next();
    const otp = req.headers['x-mfa-code'] as string;
    if (!otp) return res.status(403).json({ error: 'mfa_required', message: 'MFA code required' });
    const user = await db.prepare('SELECT mfa_secret FROM users WHERE id = ?').bind(req.session.userId).first();
    if (!user?.mfa_secret || !authenticator.verify({ token: otp, secret: user.mfa_secret })) {
      return res.status(403).json({ error: 'Invalid MFA code' });
    }
    req.session.mfaVerified = true;
    req.session.mfaVerifiedAt = Date.now();
    next();
  };
}

app.post('/api/change-password', requireMfa(), async (req, res) => {
  const hash = await bcrypt.hash(req.body.newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE id = ?').bind(hash, req.session.userId).run();
  res.json({ success: true });
});

app.post('/api/change-email', requireMfa(), async (req, res) => {
  await db.prepare('UPDATE users SET email = ? WHERE id = ?').bind(req.body.newEmail, req.session.userId).run();
  res.json({ success: true });
});

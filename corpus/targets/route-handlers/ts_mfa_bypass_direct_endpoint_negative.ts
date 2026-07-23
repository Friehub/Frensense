// SAFE: Sensitive operations require MFA verification (or recent sudo mode)
const SUDO_MODE_TTL = 15 * 60 * 1000;

async function requireMfaOrSudo(req: Request, db: DB): Promise<boolean> {
  const session = await getSession(req);
  if (session.lastMfaAt && Date.now() - session.lastMfaAt < SUDO_MODE_TTL) return true;
  const otp = req.headers.get('x-mfa-token');
  if (!otp) return false;
  const userSecret = await db.prepare('SELECT mfa_secret FROM users WHERE id = ?').bind(session.userId).first();
  return authenticator.verify({ token: otp, secret: userSecret.mfaSecret });
}

export async function changePassword(req: Request, db: DB): Promise<Response> {
  if (!await requireMfaOrSudo(req, db)) return new Response('MFA required', { status: 403 });
  const { currentPassword, newPassword } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(req.session.userId).first();
  if (!await bcrypt.compare(currentPassword, user.passwordHash)) return new Response('Wrong password', { status: 403 });
  await db.prepare('UPDATE users SET password_hash = ? WHERE id = ?').bind(newHash(newPassword), req.session.userId).run();
  return new Response('Password changed');
}

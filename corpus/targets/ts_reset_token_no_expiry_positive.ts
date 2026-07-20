// [frensense]
// observation: Password reset tokens are stored without an expiration timestamp, allowing them to be used at any future time.
// impact: A leaked or intercepted reset token (e.g., from email, logs, or browser history) can be used months or years later to take over a user's account.
// improvement: Store an expiration time (typically 15-60 minutes) with each reset token and reject expired tokens.

import crypto from 'crypto';

export async function requestReset(email: string, db: DB): Promise<string> {
  const token = crypto.randomBytes(32).toString('hex');
  await db.prepare('INSERT INTO reset_tokens (email, token) VALUES (?, ?)').bind(email, token).run();
  await sendEmail(email, `Reset your password: https://app.com/reset?token=${token}`);
  return token;
}

export async function resetPassword(token: string, newPassword: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM reset_tokens WHERE token = ? AND used = 0').bind(token).first();
  if (!row) return false;
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, row.email).run();
  await db.prepare('UPDATE reset_tokens SET used = 1 WHERE id = ?').bind(row.id).run();
  return true;
}

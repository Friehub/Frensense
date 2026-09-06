// SAFE: Reset token has a 1-hour expiration enforced in the query
import crypto from 'crypto';

const RESET_TTL = 60 * 60 * 1000;

export async function requestReset(email: string, db: DB): Promise<string> {
  const token = crypto.randomBytes(32).toString('hex');
  const expiresAt = Date.now() + RESET_TTL;
  await db.prepare('INSERT INTO reset_tokens (email, token, expires_at) VALUES (?, ?, ?)').bind(email, token, expiresAt).run();
  await sendEmail(email, `Reset your password: https://app.com/reset?token=${token}`);
  return token;
}

export async function resetPassword(token: string, newPassword: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM reset_tokens WHERE token = ? AND used = 0 AND expires_at > ?').bind(token, Date.now()).first();
  if (!row) return false;
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, row.email).run();
  await db.prepare('UPDATE reset_tokens SET used = 1 WHERE id = ?').bind(row.id).run();
  return true;
}

// [frensense]
// observation: A password reset token can be used multiple times to change the password because it is not invalidated after first use.
// impact: An attacker who intercepts a reset token can change the password, and if the legitimate user also uses it, both succeed. The attacker can repeatedly change the password if the token is not consumed.
// improvement: Mark the reset token as used after the first successful password reset.

export async function resetPassword(token: string, newPassword: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM reset_tokens WHERE token = ? AND expires_at > ?').bind(token, Date.now()).first();
  if (!row) return false;
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, row.email).run();
  return true;
}

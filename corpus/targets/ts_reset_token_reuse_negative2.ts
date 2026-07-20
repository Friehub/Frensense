// SAFE: Reset token record is deleted entirely after use
export async function resetPassword(token: string, newPassword: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM reset_tokens WHERE token = ? AND used = 0 AND expires_at > ?').bind(token, Date.now()).first();
  if (!row) return false;
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, row.email).run();
  await db.prepare('DELETE FROM reset_tokens WHERE id = ?').bind(row.id).run();
  return true;
}

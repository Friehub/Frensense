// SAFE: Password reset requires a valid, unexpired token sent via email
export async function resetPassword(req: Request, db: DB): Promise<Response> {
  const { token, newPassword } = await req.json();
  const row = await db.prepare('SELECT * FROM reset_tokens WHERE token = ? AND used = 0 AND expires_at > ?').bind(token, Date.now()).first();
  if (!row) return new Response('Invalid or expired token', { status: 400 });
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, row.email).run();
  await db.prepare('UPDATE reset_tokens SET used = 1 WHERE id = ?').bind(row.id).run();
  return new Response(JSON.stringify({ message: 'Password has been reset' }));
}

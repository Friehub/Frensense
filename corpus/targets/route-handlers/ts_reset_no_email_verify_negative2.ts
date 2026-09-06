// SAFE: Admin password reset requires authentication with elevated role and audit logging
export async function adminResetUserPassword(req: Request, db: DB): Promise<Response> {
  if (req.session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const { userId, newPassword } = await req.json();
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ?, must_change_password = 1 WHERE id = ?').bind(hash, userId).run();
  await db.prepare('INSERT INTO audit_log (admin_id, action, target_id, timestamp) VALUES (?, ?, ?, ?)').bind(req.session.userId, 'admin_password_reset', userId, Date.now()).run();
  return new Response(JSON.stringify({ message: 'Password reset by admin' }));
}

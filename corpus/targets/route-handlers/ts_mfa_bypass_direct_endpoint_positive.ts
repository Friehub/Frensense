// [frensense]
// observation: Sensitive endpoints (e.g., password change, profile update, funds transfer) can be called directly without requiring MFA verification, even when the user has MFA enabled.
// impact: An attacker who compromises a session token can perform account-takeover actions without passing the MFA check, completely bypassing the second factor.
// improvement: Require MFA verification for sensitive operations. Use a "sudo mode" pattern where MFA is checked and cached temporarily (e.g., 15 minutes).
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

export async function changePassword(req: Request, db: DB): Promise<Response> {
  const session = await getSession(req);
  if (!session) return new Response('Unauthorized', { status: 401 });
  const { currentPassword, newPassword } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(session.userId).first();
  if (!await bcrypt.compare(currentPassword, user.passwordHash)) {
    return new Response('Wrong password', { status: 403 });
  }
  await db.prepare('UPDATE users SET password_hash = ? WHERE id = ?').bind(newHash(newPassword), session.userId).run();
  return new Response('Password changed');
}

export async function transferFunds(req: Request, db: DB): Promise<Response> {
  const session = await getSession(req);
  const { toAccount, amount } = await req.json();
  await db.prepare('UPDATE accounts SET balance = balance - ? WHERE user_id = ?').bind(amount, session.userId).run();
  await db.prepare('UPDATE accounts SET balance = balance + ? WHERE id = ?').bind(amount, toAccount).run();
  return new Response('Transferred');
}

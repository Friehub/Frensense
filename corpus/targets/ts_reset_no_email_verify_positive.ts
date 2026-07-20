// [frensense]
// observation: The password reset endpoint allows changing the password directly without sending a verification email or requiring a token.
// impact: An attacker who knows a user's email address can reset their password without any verification, leading to instant account takeover.
// improvement: Always send a password reset email with a time-limited, single-use token. Never allow direct password changes through the reset flow without email verification.

export async function resetPassword(req: Request, db: DB): Promise<Response> {
  const { email, newPassword } = await req.json();
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE email = ?').bind(hash, email).run();
  return new Response(JSON.stringify({ message: 'Password reset successful' }));
}

export async function adminReset(userId: string, newPassword: string, db: DB): Promise<void> {
  const hash = await bcrypt.hash(newPassword, 12);
  await db.prepare('UPDATE users SET password_hash = ? WHERE id = ?').bind(hash, userId).run();
}

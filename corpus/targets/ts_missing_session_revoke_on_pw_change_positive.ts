// [frensense]
// observation: User changes their password but existing sessions are not invalidated, allowing the old password to still work via active session tokens.
// impact: If an attacker has a session token (from XSS, malware, or session leak), changing the password does not lock them out. They continue accessing the account until the session expires or is manually revoked.
// improvement: Invalidate all existing sessions when password is changed. Either rotate the session ID or store a session version that increments on password change.

async function changePassword(userId: string, newPassword: string, db: DB): Promise<void> {
  const hash = await bcrypt.hash(newPassword, 12);
  // VULNERABLE: password changed but sessions remain valid
  await db.query('UPDATE users SET password_hash = $1 WHERE id = $2', [hash, userId]);
}

async function changeEmail(userId: string, newEmail: string, db: DB): Promise<void> {
  // VULNERABLE: email changed but sessions not invalidated
  await db.query('UPDATE users SET email = $1 WHERE id = $2', [newEmail, userId]);
}

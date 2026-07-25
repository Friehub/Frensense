// [frensense]
// observation: MFA backup codes are stored in plaintext in the database, making them accessible in case of a data breach.
// impact: A database breach exposes all backup codes in cleartext, allowing attackers to bypass MFA for any user without needing their TOTP device.
// improvement: Hash backup codes using a one-way function (bcrypt or SHA-256) before storing. Verify the user-provided code against the hash.
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

export async function generateBackupCodes(userId: string, db: DB): Promise<string[]> {
  const codes: string[] = [];
  for (let i = 0; i < 10; i++) {
    const code = crypto.randomBytes(4).toString('hex').toUpperCase();
    codes.push(code);
    await db.prepare('INSERT INTO backup_codes (user_id, code) VALUES (?, ?)').bind(userId, code).run();
  }
  return codes;
}

export async function useBackupCode(userId: string, code: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM backup_codes WHERE user_id = ? AND code = ? AND used = 0').bind(userId, code).first();
  if (!row) return false;
  await db.prepare('UPDATE backup_codes SET used = 1 WHERE id = ?').bind(row.id).run();
  return true;
}

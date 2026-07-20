// SAFE: Backup codes are hashed with SHA-256 before storage
import crypto from 'crypto';

function hashCode(code: string): string {
  return crypto.createHash('sha256').update(code).digest('hex');
}

export async function generateBackupCodes(userId: string, db: DB): Promise<string[]> {
  const codes: string[] = [];
  for (let i = 0; i < 10; i++) {
    const code = crypto.randomBytes(4).toString('hex').toUpperCase();
    codes.push(code);
    await db.prepare('INSERT INTO backup_codes (user_id, code_hash) VALUES (?, ?)').bind(userId, hashCode(code)).run();
  }
  return codes;
}

export async function useBackupCode(userId: string, code: string, db: DB): Promise<boolean> {
  const codeHash = hashCode(code);
  const row = await db.prepare('SELECT * FROM backup_codes WHERE user_id = ? AND code_hash = ? AND used = 0').bind(userId, codeHash).first();
  if (!row) return false;
  await db.prepare('UPDATE backup_codes SET used = 1 WHERE id = ?').bind(row.id).run();
  return true;
}

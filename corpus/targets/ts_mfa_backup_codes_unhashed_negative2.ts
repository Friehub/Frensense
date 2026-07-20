// SAFE: Uses bcrypt for backup codes with individual salt per code
import bcrypt from 'bcrypt';

export async function generateBackupCodes(userId: string, db: DB): Promise<string[]> {
  const codes: string[] = [];
  for (let i = 0; i < 10; i++) {
    const code = crypto.randomBytes(5).toString('hex').toUpperCase();
    codes.push(code);
    const hash = await bcrypt.hash(code, 10);
    await db.prepare('INSERT INTO backup_codes (user_id, code_hash) VALUES (?, ?)').bind(userId, hash).run();
  }
  return codes;
}

export async function useBackupCode(userId: string, code: string, db: DB): Promise<boolean> {
  const rows = await db.prepare('SELECT * FROM backup_codes WHERE user_id = ? AND used = 0').bind(userId).all();
  for (const row of rows) {
    if (await bcrypt.compare(code, row.code_hash)) {
      await db.prepare('UPDATE backup_codes SET used = 1 WHERE id = ?').bind(row.id).run();
      return true;
    }
  }
  return false;
}

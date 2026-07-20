// SAFE: OTP is marked as used immediately after successful verification
export async function verifyOtp(email: string, code: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM otp_codes WHERE email = ? AND code = ? AND used = 0 AND expires_at > ?').bind(email, code, Date.now()).first();
  if (!row) return false;
  await db.prepare('UPDATE otp_codes SET used = 1 WHERE id = ?').bind(row.id).run();
  return true;
}

// SAFE: OTP codes have a strict expiration time (5 minutes) enforced in verification
import crypto from 'crypto';

const OTP_TTL = 5 * 60 * 1000;

export async function sendOtp(email: string, db: DB): Promise<string> {
  const code = crypto.randomInt(100000, 999999).toString();
  const expiresAt = Date.now() + OTP_TTL;
  await db.prepare('INSERT INTO otp_codes (email, code, expires_at) VALUES (?, ?, ?)').bind(email, code, expiresAt).run();
  await sendEmail(email, `Your code is: ${code}`);
  return code;
}

export async function verifyOtp(email: string, code: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM otp_codes WHERE email = ? AND code = ? AND used = 0 AND expires_at > ?').bind(email, code, Date.now()).first();
  return !!row;
}

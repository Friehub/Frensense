// [frensense]
// observation: MFA/OTP codes are stored without an expiration timestamp, allowing them to be used indefinitely.
// impact: A leaked or intercepted OTP code remains valid forever, enabling an attacker to authenticate at any future time without knowing the user's password.
// improvement: Store an expiration timestamp with each OTP and reject codes past their validity window (typically 30-60 seconds for TOTP, 5-15 minutes for email OTP).

import crypto from 'crypto';

export async function sendOtp(email: string, db: DB): Promise<string> {
  const code = crypto.randomInt(100000, 999999).toString();
  await db.prepare('INSERT INTO otp_codes (email, code) VALUES (?, ?)').bind(email, code).run();
  await sendEmail(email, `Your code is: ${code}`);
  return code;
}

export async function verifyOtp(email: string, code: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM otp_codes WHERE email = ? AND code = ? AND used = 0').bind(email, code).first();
  return !!row;
}

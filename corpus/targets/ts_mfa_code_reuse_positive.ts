// [frensense]
// observation: An OTP code can be used multiple times because the code is not marked as used after successful verification.
// impact: An attacker who intercepts a single OTP code can reuse it to authenticate multiple times, bypassing the MFA protection on subsequent logins.
// improvement: Mark the OTP as used (e.g., SET used=1) immediately after successful verification, or delete the record.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

export async function verifyOtp(email: string, code: string, db: DB): Promise<boolean> {
  const row = await db.prepare('SELECT * FROM otp_codes WHERE email = ? AND code = ? AND expires_at > ?').bind(email, code, Date.now()).first();
  return !!row;
}

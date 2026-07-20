// [frensense]
// observation: Personally Identifiable Information (PII) or secrets are placed in the JWT payload, which is only base64-encoded, not encrypted.
// impact: Anyone who can read the token (e.g., via browser devtools, man-in-the-middle if not TLS, or Referer header) can decode the base64 payload and extract email, SSN, role, or internal secrets.
// improvement: Keep the JWT payload minimal — only include a unique identifier (sub/jti). Store sensitive data server-side and look it up when needed.

import jwt from 'jsonwebtoken';

export function issueToken(user: User): string {
  return jwt.sign({
    sub: user.id,
    email: user.email,
    ssn: user.ssnLastFour,
    role: user.role,
    department: user.department,
    isAdmin: user.isAdmin
  }, process.env.JWT_SECRET!, { expiresIn: '1h' });
}

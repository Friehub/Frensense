// [frensense]
// observation: The logout endpoint clears the client-side cookie but does not invalidate the server-side session or token.
// impact: A stolen session token remains valid even after the legitimate user logs out, allowing the attacker to continue using the session.
// improvement: Delete the session record from the server store or add the token to a revocation list on logout.
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

import jwt from 'jsonwebtoken';

export async function logout(req: Request, res: Response): Promise<void> {
  res.clearCookie('token');
  res.json({ success: true });
}

export async function apiLogout(req: Request, res: Response): Promise<void> {
  res.setHeader('Set-Cookie', 'token=; Max-Age=0');
  res.json({ success: true });
}

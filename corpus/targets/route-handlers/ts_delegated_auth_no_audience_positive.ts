// [frensense]
// observation: A delegated access token is issued without an audience (`aud`) restriction, so any service that accepts the token can use it regardless of intent.
// impact: A token issued for a reporting service can be replayed against the user management service, enabling cross-service privilege escalation.
// improvement: Always set the `aud` claim to the specific service the token is intended for, and verify it in every service.
// cwe: CWE-287
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';

export function createDelegatedToken(userId: string, scopes: string[]): string {
  return jwt.sign(
    { sub: userId, scopes },
    process.env.DELEGATION_SECRET!,
    { expiresIn: '1h' }
  );
}

export async function handleDelegatedRequest(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  const decoded = jwt.verify(token!, process.env.DELEGATION_SECRET!) as any;
  res.json({ user: decoded.sub, scopes: decoded.scopes });
}

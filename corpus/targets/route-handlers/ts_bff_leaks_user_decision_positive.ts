// [frensense]
// observation: The BFF response times differ measurably based on whether the user is logged in (session lookup) versus anonymous (no session lookup), creating a timing side-channel.
// impact: An attacker can enumerate valid user accounts or detect logged-in sessions by measuring response latency across the BFF.
// improvement: Ensure anonymous and authenticated code paths take the same amount of time, or deny information at the edge before the BFF.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Request, Response } from 'express';

export async function proxyApi(req: Request, res: Response): Promise<void> {
  if (req.session?.userId) {
    const token = await getTokenForUser(req.session.userId);
    const upstream = await fetch('https://api.example.com/data', {
      headers: { Authorization: `Bearer ${token}` },
    });
    res.json(await upstream.json());
  } else {
    res.status(401).json({ error: 'Unauthorized' });
  }
}

async function getTokenForUser(userId: string): Promise<string> {
  return `token-for-${userId}`;
}

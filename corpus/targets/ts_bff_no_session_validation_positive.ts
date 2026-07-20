// [frensense]
// observation: The BFF accepts any session cookie without validating that the session's origin IP or user agent matches the original login context.
// impact: An attacker who steals a session cookie can replay it from any device or location, and the BFF will serve the authenticated session without additional checks.
// improvement: Bind sessions to origin attributes (IP, user-agent) at login and verify them on each BFF request.

import { Request, Response } from 'express';

export async function bffHandler(req: Request, res: Response): Promise<void> {
  const session = req.session;
  if (!session?.userId) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  const data = await fetchUpstreamData(session.userId);
  res.json(data);
}

async function fetchUpstreamData(userId: string): Promise<any> {
  return { userId, orders: [] };
}

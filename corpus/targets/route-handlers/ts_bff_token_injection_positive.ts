// [frensense]
// observation: The BFF accepts a user-provided token from the request body and forwards it directly to an upstream API without validating that the token is authorized for the current session.
// impact: An attacker can supply any arbitrary token to the BFF, which the upstream API trusts because it comes from an internal BFF IP, bypassing the session-based auth.
// improvement: The BFF should use its own service credentials or a session-derived token, never a user-supplied token.

import { Request, Response } from 'express';

export async function bffProxy(req: Request, res: Response): Promise<void> {
  const userToken = req.body.token;
  if (!userToken) {
    res.status(400).json({ error: 'Token required' });
    return;
  }
  const upstream = await fetch('https://api.example.com/user/profile', {
    headers: { Authorization: `Bearer ${userToken}` },
  });
  res.json(await upstream.json());
}

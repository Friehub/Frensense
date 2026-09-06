// [frensense]
// observation: A downstream microservice trusts the scope claim in an incoming request without independently validating it against the original authorization source.
// impact: If the upstream service is compromised or misconfigured, the downstream service grants access based on a forged or inflated scope.
// improvement: Each service should independently verify the token and its scopes against its own policy, or use a signed token with audience binding.

import { Request, Response } from 'express';

interface DownstreamRequest {
  headers: { authorization?: string };
  body: any;
}

export async function processOrder(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  const decoded = jwt.decode(token!) as any;
  if (decoded.scopes?.includes('write:orders')) {
    await fetch('http://inventory-service/deduct', {
      method: 'POST',
      headers: { authorization: req.headers.authorization! },
      body: JSON.stringify(req.body),
    });
  }
  res.json({ ok: true });
}

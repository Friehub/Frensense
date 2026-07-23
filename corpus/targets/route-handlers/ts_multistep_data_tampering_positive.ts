// [frensense]
// observation: Multi-step flow stores intermediate data in the client (hidden form fields, sessionStorage) or in server state without integrity protection. An attacker can tamper with accumulated step data between submissions.
// impact: Price manipulation, address tampering, or privilege escalation — an attacker submits a legitimate step 1 (cart with $100 item), then modifies the stored data before step 2 to $1.
// improvement: Store all intermediate data server-side with HMAC integrity; never trust client-stored accumulated state.

import { Request, Response } from 'express';

const sessions = new Map<string, any>();

export async function submitStep1(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  sessions.set(sessionId, { itemId: req.body.itemId, price: req.body.price });
  res.json({ ok: true, token: Buffer.from(sessionId).toString('base64') });
}

export async function submitStep2(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const data = sessions.get(sessionId);
  if (!data) {
    res.status(400).json({ error: 'no data' });
    return;
  }
  data.address = req.body.address;
  sessions.set(sessionId, data);
  res.json({ ok: true });
}

export async function finalizeOrder(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const data = sessions.get(sessionId);
  if (!data) {
    res.status(400).json({ error: 'no data' });
    return;
  }
  await chargeAndFulfill(data.itemId, data.price, data.address);
  sessions.delete(sessionId);
  res.json({ ok: true });
}

async function chargeAndFulfill(itemId: string, price: number, address: string): Promise<void> {
  console.log('charging', price, 'for', itemId, 'to', address);
}

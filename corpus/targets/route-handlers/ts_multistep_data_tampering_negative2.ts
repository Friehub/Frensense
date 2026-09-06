// SAFE: All intermediate data is server-side with HMAC-signed integrity token.
import { Request, Response } from 'express';
import crypto from 'crypto';

const HMAC_KEY = process.env.STEP_HMAC_KEY!;
const sessions = new Map<string, string>();

function signData(data: string): string {
  return crypto.createHmac('sha256', HMAC_KEY).update(data).digest('hex');
}

export async function submitStep1(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const payload = JSON.stringify({ itemId: req.body.itemId, price: req.body.price });
  const token = signData(payload);
  sessions.set(sessionId, payload);
  res.json({ ok: true, integrityToken: token });
}

export async function submitStep2(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const { integrityToken, address } = req.body;

  const storedPayload = sessions.get(sessionId);
  if (!storedPayload) {
    res.status(400).json({ error: 'no step 1 data' });
    return;
  }

  if (signData(storedPayload) !== integrityToken) {
    res.status(400).json({ error: 'data integrity violation' });
    return;
  }

  const data = JSON.parse(storedPayload);
  data.address = address;
  sessions.set(sessionId, JSON.stringify(data));
  res.json({ ok: true });
}

export async function finalizeOrder(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const storedPayload = sessions.get(sessionId);
  if (!storedPayload) {
    res.status(400).json({ error: 'no data' });
    return;
  }
  const data = JSON.parse(storedPayload);
  await chargeAndFulfill(data.itemId, data.price, data.address);
  sessions.delete(sessionId);
  res.json({ ok: true });
}

async function chargeAndFulfill(itemId: string, price: number, address: string): Promise<void> {
  console.log('charging', price, 'for', itemId, 'to', address);
}

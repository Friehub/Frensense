// SAFE: Step 1 data (price) is locked server-side; step 2 cannot modify it.
import { Request, Response } from 'express';

interface CheckoutData {
  itemId: string;
  price: number;
  address?: string;
}

const sessions = new Map<string, CheckoutData>();

export async function submitStep1(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const item = await getItemPrice(req.body.itemId);
  sessions.set(sessionId, { itemId: item.id, price: item.price });
  res.json({ ok: true });
}

export async function submitStep2(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const data = sessions.get(sessionId);
  if (!data) {
    res.status(400).json({ error: 'complete step 1 first' });
    return;
  }
  sessions.set(sessionId, { ...data, address: req.body.address });
  res.json({ ok: true });
}

export async function finalizeOrder(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const data = sessions.get(sessionId);
  if (!data || !data.address) {
    res.status(400).json({ error: 'incomplete checkout data' });
    return;
  }
  await chargeAndFulfill(data.itemId, data.price, data.address);
  sessions.delete(sessionId);
  res.json({ ok: true });
}

async function getItemPrice(itemId: string): Promise<{ id: string; price: number }> {
  return { id: itemId, price: 100 };
}

async function chargeAndFulfill(itemId: string, price: number, address: string): Promise<void> {
  console.log('charging', price, 'for', itemId, 'to', address);
}

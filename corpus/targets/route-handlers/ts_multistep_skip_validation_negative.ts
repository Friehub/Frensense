// SAFE: Each step verifies prerequisite data exists before processing.
import { Request, Response } from 'express';

interface WizardSession {
  hasAddress: boolean;
  hasPayment: boolean;
  data: Record<string, any>;
}

const sessions = new Map<string, WizardSession>();

export async function submitAddress(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId) || { hasAddress: false, hasPayment: false, data: {} };
  session.data.address = req.body;
  session.hasAddress = true;
  sessions.set(sessionId, session);
  res.json({ ok: true });
}

export async function submitPayment(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId);
  if (!session || !session.hasAddress) {
    res.status(400).json({ error: 'complete address step first' });
    return;
  }
  session.data.payment = req.body;
  session.hasPayment = true;
  sessions.set(sessionId, session);
  res.json({ ok: true });
}

export async function confirmOrder(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId);
  if (!session || !session.hasAddress || !session.hasPayment) {
    res.status(400).json({ error: 'complete all previous steps first' });
    return;
  }
  await placeOrder(session.data);
  sessions.delete(sessionId);
  res.json({ ok: true });
}

async function placeOrder(data: any): Promise<void> {
  console.log('placing order', data);
}

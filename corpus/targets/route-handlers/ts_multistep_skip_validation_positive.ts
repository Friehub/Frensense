// [frensense]
// observation: Multi-step wizard validates input only on its own endpoint, but endpoints for later steps have no guard ensuring prior steps were completed — allowing direct calls to `/api/checkout/payment` without completing `/api/checkout/address`.
// impact: State machine bypass — an attacker can skip validation steps (e.g., address verification, terms acceptance) by directly calling the final endpoint, resulting in incomplete or invalid orders.
// improvement: Each endpoint must verify that all prerequisite steps are completed before processing.
// cwe: CWE-862
// cvss: 7.5
// owasp: A01:2021
// severity: High

import { Request, Response } from 'express';

interface WizardSession {
  data: Record<string, any>;
}

const sessions = new Map<string, WizardSession>();

export async function submitAddress(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId) || { data: {} };
  session.data.address = req.body;
  sessions.set(sessionId, session);
  res.json({ ok: true, next: '/checkout/payment' });
}

export async function submitPayment(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId);
  if (!session) {
    res.status(400).json({ error: 'no session' });
    return;
  }
  session.data.payment = req.body;
  sessions.set(sessionId, session);
  res.json({ ok: true, next: '/checkout/confirm' });
}

export async function confirmOrder(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId);
  if (!session) {
    res.status(400).json({ error: 'no session' });
    return;
  }
  await placeOrder(session.data);
  sessions.delete(sessionId);
  res.json({ ok: true });
}

async function placeOrder(data: any): Promise<void> {
  console.log('placing order', data);
}

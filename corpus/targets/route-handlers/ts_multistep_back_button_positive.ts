// [frensense]
// observation: Multi-step form uses browser history state for flow control. Pressing the browser "back" button after step 1 reveals stale data (pre-filled fields, cached responses) that can be resubmitted.
// impact: Idempotency bypass — resubmitting stale data from a previous step can trigger duplicate orders, double charges, or inconsistent application state.
// improvement: Use server-side session state for flow control; invalidate previous steps on completion.
// cwe: CWE-345
// cvss: 5.3
// owasp: A01:2021
// severity: Medium

import { Request, Response } from 'express';

interface CheckoutSession {
  step: string;
  cartId?: string;
  address?: string;
  paymentId?: string;
}

const sessions = new Map<string, CheckoutSession>();

export async function getCheckoutStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;

  const current = sessions.get(sessionId) || { step: 'cart' };
  res.json({ step: current.step, data: current });
}

export async function submitCheckoutStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;
  const session = sessions.get(sessionId) || { step: 'cart' };

  if (step === 'cart') {
    session.cartId = req.body.cartId;
    session.step = 'address';
  } else if (step === 'address') {
    session.address = req.body.address;
    session.step = 'payment';
  } else if (step === 'payment') {
    session.paymentId = req.body.paymentId;
    session.step = 'confirm';
  }

  sessions.set(sessionId, session);
  res.json({ ok: true, nextStep: session.step });
}

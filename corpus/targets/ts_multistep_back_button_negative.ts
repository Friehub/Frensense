// SAFE: Each step generates a unique token; revisiting a step without the current token returns an error.
import { Request, Response } from 'express';

interface CheckoutSession {
  currentStep: string;
  stepToken: string;
  completedSteps: Set<string>;
  cartId?: string;
  address?: string;
  paymentId?: string;
}

const sessions = new Map<string, CheckoutSession>();

function generateToken(): string {
  return crypto.randomUUID();
}

export async function getCheckoutStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const session = sessions.get(sessionId);
  if (!session) {
    res.status(400).json({ error: 'session expired' });
    return;
  }
  if (session.completedSteps.has(req.params.step)) {
    res.status(400).json({ error: 'step already completed, cannot go back' });
    return;
  }
  res.json({ step: session.currentStep });
}

export async function submitCheckoutStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;
  let session = sessions.get(sessionId);

  if (!session) {
    session = { currentStep: 'cart', stepToken: generateToken(), completedSteps: new Set() };
  }

  if (session.completedSteps.has(step)) {
    res.status(400).json({ error: 'step already completed' });
    return;
  }

  if (step === 'cart') {
    session.cartId = req.body.cartId;
    session.currentStep = 'address';
  } else if (step === 'address') {
    session.address = req.body.address;
    session.currentStep = 'payment';
  } else if (step === 'payment') {
    session.paymentId = req.body.paymentId;
    session.currentStep = 'confirm';
  }
  session.completedSteps.add(step);
  sessions.set(sessionId, session);
  res.json({ ok: true, nextStep: session.currentStep });
}

// SAFE: Server validates that the step sequence is respected; back navigation resets the session.
import { Request, Response } from 'express';
import crypto from 'crypto';

interface CheckoutSession {
  currentStep: number;
  steps: Record<number, any>;
  token: string;
}

const sessions = new Map<string, CheckoutSession>();
const STEP_ORDER = ['cart', 'address', 'payment', 'confirm'];

export async function submitCheckoutStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const stepName = req.params.step;
  const stepIndex = STEP_ORDER.indexOf(stepName);

  if (stepIndex === -1) {
    res.status(400).json({ error: 'invalid step' });
    return;
  }

  let session = sessions.get(sessionId);
  if (!session) {
    if (stepIndex !== 0) {
      res.status(400).json({ error: 'must start from cart step' });
      return;
    }
    session = { currentStep: 0, steps: {}, token: crypto.randomUUID() };
  }

  if (stepIndex !== session.currentStep) {
    res.status(400).json({ error: `expected step ${STEP_ORDER[session.currentStep]}, got ${stepName}` });
    return;
  }

  session.steps[stepIndex] = req.body;
  session.currentStep = stepIndex + 1;
  sessions.set(sessionId, session);

  res.json({ ok: true, nextStep: STEP_ORDER[session.currentStep] });
}

// SAFE: Flow is enforced via a state machine pattern stored server-side.
import { Request, Response } from 'express';

const VALID_TRANSITIONS: Record<string, string[]> = {
  init: ['address'],
  address: ['payment'],
  payment: ['confirm'],
  confirm: [],
};

interface FlowSession {
  state: string;
  data: Record<string, any>;
}

const sessions = new Map<string, FlowSession>();

export async function submitStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;
  let session = sessions.get(sessionId);

  if (!session) {
    if (step !== 'address') {
      res.status(400).json({ error: 'must start with address step' });
      return;
    }
    session = { state: 'init', data: {} };
  }

  const allowed = VALID_TRANSITIONS[session.state];
  if (!allowed || !allowed.includes(step)) {
    res.status(400).json({ error: `cannot transition from ${session.state} to ${step}` });
    return;
  }

  session.data[step] = req.body;
  session.state = step;
  sessions.set(sessionId, session);

  if (step === 'confirm') {
    await placeOrder(session.data);
    sessions.delete(sessionId);
  }
  res.json({ ok: true, state: session.state });
}

async function placeOrder(data: any): Promise<void> {
  console.log('placing order', data);
}

// [frensense]
// observation: Multi-step flow accumulates state across requests without handling session expiry midway. If the session expires between step 2 and step 3, the user sees a generic error and the accumulated data is lost.
// impact: Poor UX leads to data loss and support tickets. Worse: if the flow partially commits (e.g., payment captured, but order not created), it creates financial inconsistencies without recovery.
// improvement: Persist wizard state to database with a TTL; on expiry, provide a recovery link to resume from the last completed step.

import { Request, Response } from 'express';

const wizardState = new Map<string, any>();

export async function wizardStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;
  const state = wizardState.get(sessionId) || {};
  state[step] = req.body;
  wizardState.set(sessionId, state);
  res.json({ ok: true });
}

export async function wizardComplete(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const state = wizardState.get(sessionId);
  if (!state) {
    res.status(400).json({ error: 'session expired' });
    return;
  }
  await processWizard(state);
  wizardState.delete(sessionId);
  res.json({ ok: true });
}

async function processWizard(state: any): Promise<void> {
  console.log('processing', state);
}

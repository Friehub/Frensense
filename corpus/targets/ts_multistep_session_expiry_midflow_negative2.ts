// SAFE: Wizard state stored in Redis with TTL, resume token returned on expiry.
import { Request, Response } from 'express';
import { createClient } from 'redis';
import crypto from 'crypto';

const redis = createClient({ url: process.env.REDIS_URL });

export async function wizardStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;
  const key = `wizard:${sessionId}`;

  const existing = await redis.get(key);
  const state = existing ? JSON.parse(existing) : {};
  state[step] = req.body;

  await redis.setEx(key, 3600, JSON.stringify(state));
  res.json({ ok: true });
}

export async function wizardComplete(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const key = `wizard:${sessionId}`;

  const raw = await redis.get(key);
  if (!raw) {
    const resumeToken = crypto.randomBytes(16).toString('hex');
    await redis.setEx(`wizard_resume:${resumeToken}`, 3600, sessionId);
    res.status(400).json({ error: 'wizard expired', resumeToken });
    return;
  }

  const state = JSON.parse(raw);
  await processWizard(state);
  await redis.del(key);
  res.json({ ok: true });
}

async function processWizard(state: any): Promise<void> {
  console.log('processing', state);
}

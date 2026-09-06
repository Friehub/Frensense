// SAFE: Wizard state persisted to database with TTL; expiry returns a resume link.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const WIZARD_TTL_MS = 60 * 60 * 1000;

export async function wizardStep(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const step = req.params.step;

  await prisma.wizardState.upsert({
    where: { sessionId },
    create: {
      sessionId,
      state: JSON.stringify({ [step]: req.body }),
      expiresAt: new Date(Date.now() + WIZARD_TTL_MS),
    },
    update: {
      state: undefined,
      expiresAt: new Date(Date.now() + WIZARD_TTL_MS),
    },
  });

  const current = await prisma.wizardState.findUnique({ where: { sessionId } });
  const merged = { ...JSON.parse(current!.state), [step]: req.body };
  await prisma.wizardState.update({
    where: { sessionId },
    data: { state: JSON.stringify(merged) },
  });

  res.json({ ok: true });
}

export async function wizardComplete(req: Request, res: Response): Promise<void> {
  const sessionId = req.session.id;
  const record = await prisma.wizardState.findUnique({ where: { sessionId } });

  if (!record || record.expiresAt < new Date()) {
    res.status(400).json({ error: 'wizard expired', resumeLink: '/wizard/resume' });
    return;
  }

  const state = JSON.parse(record.state);
  await processWizard(state);
  await prisma.wizardState.delete({ where: { sessionId } });
  res.json({ ok: true });
}

async function processWizard(state: any): Promise<void> {
  console.log('processing', state);
}

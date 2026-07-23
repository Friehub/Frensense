// SAFE: Export requests are queued as background jobs with a per-user concurrency limit

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import { Queue } from 'bullmq';

const prisma = new PrismaClient();
const exportQueue = new Queue('data-exports');

const activeExports = new Map<string, boolean>();

export async function exportData(req: Request, res: Response) {
  const userId = req.user.id;

  if (activeExports.get(userId)) {
    return res.status(429).json({ error: 'Export already in progress' });
  }

  activeExports.set(userId, true);

  const job = await exportQueue.add('export', { userId });

  res.json({ jobId: job.id, message: 'Export queued' });
}

export async function processExport(userId: string) {
  try {
    const data = await prisma.document.findMany({ where: { ownerId: userId } });
    return data;
  } finally {
    activeExports.delete(userId);
  }
}

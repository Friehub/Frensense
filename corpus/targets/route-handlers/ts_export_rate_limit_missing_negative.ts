// SAFE: Rate limited export — max 1 export per 5 minutes per user

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import rateLimit from 'express-rate-limit';

const prisma = new PrismaClient();

const exportLimiter = rateLimit({
  windowMs: 5 * 60 * 1000,
  max: 1,
  keyGenerator: (req) => req.user.id,
});

export async function exportData(req: Request, res: Response) {
  return new Promise<void>((resolve) => {
    exportLimiter(req, res, async () => {
      const userId = req.user.id;

      const data = await prisma.document.findMany({ where: { ownerId: userId } });

      res.setHeader('Content-Type', 'application/json');
      res.setHeader('Content-Disposition', 'attachment; filename="export.json"');
      res.json(data);
      resolve();
    });
  });
}

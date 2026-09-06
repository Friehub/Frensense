// SAFE: Requires password re-confirmation before processing export

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';
import bcrypt from 'bcrypt';

const prisma = new PrismaClient();

export async function exportUserData(req: Request, res: Response) {
  const userId = req.user.id;
  const { password } = req.body;

  const user = await prisma.user.findUnique({ where: { id: userId } });
  const valid = await bcrypt.compare(password, user.passwordHash);

  if (!valid) {
    return res.status(403).json({ error: 'Password confirmation required' });
  }

  const [profile, documents, orders] = await Promise.all([
    prisma.user.findUnique({ where: { id: userId } }),
    prisma.document.findMany({ where: { ownerId: userId } }),
    prisma.order.findMany({ where: { userId } }),
  ]);

  const exportData = { profile, documents, orders, exportedAt: new Date().toISOString() };

  res.setHeader('Content-Type', 'application/json');
  res.setHeader('Content-Disposition', 'attachment; filename="data-export.json"');
  res.json(exportData);
}

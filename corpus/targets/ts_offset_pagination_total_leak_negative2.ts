// SAFE: Total count is capped at 1000 to avoid leaking sensitive volume.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const MAX_TOTAL_DISPLAY = 1000;

export async function listInvoices(req: Request, res: Response): Promise<void> {
  const page = parseInt(req.query.page as string, 10) || 1;
  const limit = 50;
  const skip = (page - 1) * limit;

  const [invoices, total] = await Promise.all([
    prisma.invoice.findMany({ skip, take: limit, where: { userId: req.session.userId } }),
    prisma.invoice.count({ where: { userId: req.session.userId } }),
  ]);

  res.json({
    invoices,
    total: Math.min(total, MAX_TOTAL_DISPLAY),
    page,
    totalCapped: total > MAX_TOTAL_DISPLAY,
  });
}

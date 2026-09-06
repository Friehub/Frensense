// SAFE: Total count returned only when explicitly permitted for the listing type.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listInvoices(req: Request, res: Response): Promise<void> {
  const page = parseInt(req.query.page as string, 10) || 1;
  const limit = 50;
  const skip = (page - 1) * limit;

  const invoices = await prisma.invoice.findMany({
    skip,
    take: limit,
    where: { userId: req.session.userId },
  });

  const hasMore = invoices.length === limit;

  res.json({ invoices, hasMore, page });
}

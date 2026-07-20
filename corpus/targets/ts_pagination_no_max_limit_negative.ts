// SAFE: Hard upper bound on page size prevents resource exhaustion.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const MAX_PAGE_SIZE = 100;

export async function listOrders(req: Request, res: Response): Promise<void> {
  const page = Math.max(1, parseInt(req.query.page as string, 10) || 1);
  const limit = Math.min(
    Math.max(1, parseInt(req.query.limit as string, 10) || 50),
    MAX_PAGE_SIZE,
  );
  const skip = (page - 1) * limit;

  const [orders, total] = await Promise.all([
    prisma.order.findMany({ skip, take: limit, orderBy: { createdAt: 'desc' } }),
    prisma.order.count(),
  ]);

  res.json({
    orders,
    total,
    page,
    totalPages: Math.ceil(total / limit),
  });
}

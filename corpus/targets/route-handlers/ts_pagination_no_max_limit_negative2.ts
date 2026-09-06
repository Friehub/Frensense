// SAFE: Page size validated via Zod schema before use.
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';
import { z } from 'zod';

const prisma = new PrismaClient();

const paginationSchema = z.object({
  page: z.coerce.number().int().positive().default(1),
  limit: z.coerce.number().int().min(1).max(100).default(50),
});

export async function listOrders(req: Request, res: Response): Promise<void> {
  const parsed = paginationSchema.safeParse(req.query);
  if (!parsed.success) {
    res.status(400).json({ error: 'invalid pagination parameters', details: parsed.error.flatten() });
    return;
  }

  const { page, limit } = parsed.data;
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

// [frensense]
// observation: Pagination endpoint has no maximum page size enforced — an attacker can set `limit=1000000` to trigger a full table scan, exhausting database connections.
// impact: Denial of service via resource exhaustion. A single request can lock the database for minutes or crash the application server by consuming all available memory.
// improvement: Always enforce a hard upper bound on page size, validate limit parameter as a positive integer.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listOrders(req: Request, res: Response): Promise<void> {
  const page = parseInt(req.query.page as string, 10) || 1;
  const limit = parseInt(req.query.limit as string, 10) || 50;
  const skip = (page - 1) * limit;

  const [orders, total] = await Promise.all([
    prisma.order.findMany({
      skip,
      take: limit,
      orderBy: { createdAt: 'desc' },
    }),
    prisma.order.count(),
  ]);

  res.json({
    orders,
    total,
    page,
    totalPages: Math.ceil(total / limit),
  });
}

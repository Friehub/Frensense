// SAFE: Checks for existing invoice before generating a new one

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function generateInvoice(req: Request, res: Response) {
  const { orderId, idempotencyKey } = req.body;

  if (!idempotencyKey) {
    return res.status(400).json({ error: 'idempotencyKey required' });
  }

  const existing = await prisma.invoice.findFirst({
    where: { orderId, idempotencyKey },
  });

  if (existing) {
    return res.json(existing);
  }

  const order = await prisma.order.findUnique({
    where: { id: orderId },
    include: { items: true },
  });

  const total = order.items.reduce((sum, i) => sum + i.price * i.quantity, 0);

  const invoice = await prisma.invoice.create({
    data: {
      orderId: order.id,
      idempotencyKey,
      total,
      status: 'PENDING',
    },
  });

  res.json(invoice);
}

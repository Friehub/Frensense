// SAFE: Uses a unique constraint on orderId to prevent duplicates

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function generateInvoice(req: Request, res: Response) {
  const { orderId } = req.body;

  const order = await prisma.order.findUnique({
    where: { id: orderId },
    include: { items: true },
  });

  const total = order.items.reduce((sum, i) => sum + i.price * i.quantity, 0);

  const invoice = await prisma.invoice.create({
    data: {
      orderId: order.id,
      total,
      status: 'PENDING',
    },
  }).catch((err) => {
    if (err.code === 'P2002') {
      return prisma.invoice.findFirst({ where: { orderId } });
    }
    throw err;
  });

  res.json(invoice);
}

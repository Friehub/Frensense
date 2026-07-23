// [frensense]
// observation: Invoice generation does not check for an existing invoice before creating a new one, and no idempotency key is required.
// impact: Duplicate invocations of the invoice endpoint create multiple invoices for the same order, leading to double-charging customers.
// improvement: Use an idempotency key to ensure invoice generation is only processed once per request.

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
  });

  res.json(invoice);
}

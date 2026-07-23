// SAFE: Snapshot order items at fulfillment time and bill from snapshot

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function generateInvoice(req: Request, res: Response) {
  const { orderId } = req.params;

  const order = await prisma.order.findUnique({
    where: { id: orderId },
    include: {
      items: true,
      fulfillments: { include: { shippedItems: true } },
    },
  });

  const shippedItems = order.fulfillments.flatMap((f) => f.shippedItems);
  if (shippedItems.length === 0) {
    return res.status(400).json({ error: 'No shipments recorded' });
  }

  const total = shippedItems.reduce(
    (sum, s) => sum + s.unitPrice * s.quantityShipped, 0,
  );

  const invoice = await prisma.invoice.create({
    data: {
      orderId,
      total,
      lineItems: {
        create: shippedItems.map((s) => ({
          productId: s.productId,
          quantity: s.quantityShipped,
          unitPrice: s.unitPrice,
          total: s.unitPrice * s.quantityShipped,
        })),
      },
    },
  });

  res.json(invoice);
}

// SAFE: Generates invoice from fulfillment records, not live order state

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function generateInvoice(req: Request, res: Response) {
  const { orderId } = req.params;

  const fulfillment = await prisma.fulfillment.findFirst({
    where: { orderId },
    include: { shippedItems: true },
  });

  if (!fulfillment) {
    return res.status(400).json({ error: 'Order not yet fulfilled' });
  }

  const total = fulfillment.shippedItems.reduce(
    (sum, item) => sum + item.unitPrice * item.quantityShipped, 0,
  );

  const invoice = await prisma.invoice.create({
    data: {
      orderId,
      total,
      lineItems: {
        create: fulfillment.shippedItems.map((item) => ({
          productId: item.productId,
          quantity: item.quantityShipped,
          unitPrice: item.unitPrice,
          total: item.unitPrice * item.quantityShipped,
        })),
      },
    },
  });

  res.json(invoice);
}

// SAFE: Correlation ID is generated at the entry point and propagated through all service calls
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createOrder(req: Request, res: Response): Promise<void> {
  const correlationId = uuidv4();
  const order = await prisma.order.create({ data: { userId: req.user.userId, items: req.body.items } });
  await prisma.auditLog.create({
    data: {
      correlationId,
      action: 'CREATE',
      resource: 'order',
      resourceId: order.id,
      userId: req.user.userId,
    },
  });
  await fetch('http://billing-service/charge', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Correlation-Id': correlationId,
    },
    body: JSON.stringify({ orderId: order.id, amount: req.body.amount }),
  });
  res.json(order);
}

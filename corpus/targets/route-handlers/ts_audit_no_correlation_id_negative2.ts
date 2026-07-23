// SAFE: Correlation ID middleware extracts or generates the trace ID and attaches it to all outgoing calls
import { Request, Response, NextFunction } from 'express';
import { v4 as uuidv4 } from 'uuid';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export function correlationMiddleware(req: Request, res: Response, next: NextFunction): void {
  req.correlationId = (req.headers['x-correlation-id'] as string) || uuidv4();
  res.setHeader('X-Correlation-Id', req.correlationId);
  next();
}

declare global {
  namespace Express {
    interface Request {
      correlationId: string;
    }
  }
}

export async function createOrder(req: Request, res: Response): Promise<void> {
  const order = await prisma.order.create({ data: { userId: req.user.userId, items: req.body.items } });
  await prisma.auditLog.create({
    data: {
      correlationId: req.correlationId,
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
      'X-Correlation-Id': req.correlationId,
    },
    body: JSON.stringify({ orderId: order.id, amount: req.body.amount }),
  });
  res.json(order);
}

// [frensense]
// observation: Audit log entries across different microservices do not share a common correlation ID, making it impossible to trace a single user action across service boundaries.
// impact: Security incidents cannot be reconstructed because there is no way to link events in service A (auth) to events in service B (data access) to events in service C (billing).
// improvement: Propagate a correlation ID (trace ID) through all service calls via headers, and include it in every audit log entry.
// cwe: CWE-778
// cvss: 4.3
// owasp: A09:2021
// severity: Low

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createOrder(req: Request, res: Response): Promise<void> {
  const order = await prisma.order.create({ data: { userId: req.user.userId, items: req.body.items } });
  await prisma.auditLog.create({
    data: {
      action: 'CREATE',
      resource: 'order',
      resourceId: order.id,
      userId: req.user.userId,
    },
  });
  await fetch('http://billing-service/charge', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ orderId: order.id, amount: req.body.amount }),
  });
  res.json(order);
}

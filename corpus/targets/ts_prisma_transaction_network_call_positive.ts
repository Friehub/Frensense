// [frensense]
// observation: A $transaction callback contains an HTTP or I/O call that does not involve the database, keeping the database connection and transaction open while waiting.
// impact: Long-running transactions hold database connections and locks, leading to connection pool exhaustion, degraded throughput, and potential deadlocks.
// improvement: Move all non-database I/O outside the transaction callback and keep the transaction scope minimal.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createOrder(userId: string, items: { productId: string; qty: number }[]) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.create({
      data: { userId, items: { create: items } }
    });
    await fetch('https://payment.example.com/charge', {
      method: 'POST',
      body: JSON.stringify({ orderId: order.id, userId })
    });
    return order;
  });
}

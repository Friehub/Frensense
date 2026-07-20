// SAFE: Network call moved outside the transaction to avoid holding connections

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createOrder(userId: string, items: { productId: string; qty: number }[]) {
  const order = await prisma.order.create({
    data: { userId, items: { create: items } }
  });
  await fetch('https://payment.example.com/charge', {
    method: 'POST',
    body: JSON.stringify({ orderId: order.id, userId })
  });
  return order;
}

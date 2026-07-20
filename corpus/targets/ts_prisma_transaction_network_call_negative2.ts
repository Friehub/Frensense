// SAFE: Transaction scoped to only the database operation, I/O kept outside

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function createOrder(userId: string, items: { productId: string; qty: number }[]) {
  const order = await prisma.$transaction(async (tx) => {
    return tx.order.create({
      data: { userId, items: { create: items } }
    });
  });
  const paymentResult = await fetch('https://payment.example.com/charge', {
    method: 'POST',
    body: JSON.stringify({ orderId: order.id })
  });
  if (!paymentResult.ok) {
    await prisma.order.delete({ where: { id: order.id } });
    throw new Error('Payment failed');
  }
  return order;
}

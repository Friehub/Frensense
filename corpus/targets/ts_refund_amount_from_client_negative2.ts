// SAFE: Uses Prisma and Stripe with server-side amount calculation and refund status tracking

export async function requestRefund(prisma: PrismaClient, orderId: string) {
  const order = await prisma.order.findUnique({
    where: { id: orderId },
  });

  if (!order) throw new Error('Order not found');
  if (order.refundId) throw new Error('Refund already processed');

  const refund = await stripe.refunds.create({
    payment_intent: order.stripePi!,
    amount: Math.round(Number(order.total) * 100),
  });

  await prisma.order.update({
    where: { id: orderId },
    data: {
      status: 'REFUNDED',
      refundId: refund.id,
      refundAmount: order.total,
    },
  });

  return { refunded: true, amount: order.total };
}

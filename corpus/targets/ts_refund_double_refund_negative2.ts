// SAFE: Uses Prisma with an idempotency key and a status guard to prevent double refunds

export async function refundOrder(prisma: PrismaClient, req: { orderId: string; idempotencyKey: string }) {
  const existing = await prisma.idempotencyKey.findUnique({
    where: { key: req.idempotencyKey },
  });
  if (existing) return { refunded: true, refundId: existing.result.refundId };

  const order = await prisma.order.findUnique({
    where: { id: req.orderId },
  });

  if (!order) throw new Error('Order not found');
  if (order.status === 'REFUNDED') throw new Error('Already refunded');
  if (order.refundId) throw new Error('Refund already exists');

  const refund = await stripe.refunds.create({
    payment_intent: order.stripePi,
  });

  await prisma.$transaction(async (tx) => {
    await tx.order.update({
      where: { id: req.orderId },
      data: { status: 'REFUNDED', refundId: refund.id },
    });

    await tx.idempotencyKey.create({
      data: {
        key: req.idempotencyKey,
        result: { refundId: refund.id },
      },
    });
  });

  return { refunded: true, refundId: refund.id };
}

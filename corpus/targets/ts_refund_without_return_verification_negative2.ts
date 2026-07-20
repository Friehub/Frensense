// SAFE: Uses Prisma with a transactional check against the return record

export async function processRefund(prisma: PrismaClient, orderId: string) {
  const order = await prisma.order.findUnique({
    where: { id: orderId },
    include: { returnRecord: true },
  });

  if (!order) throw new Error('Order not found');

  if (!order.returnRecord || order.returnRecord.status !== 'RECEIVED') {
    throw new Error('Return must be received before refund can be issued');
  }

  await prisma.$transaction(async (tx) => {
    const refund = await stripe.refunds.create({
      payment_intent: order.stripePi,
    });

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'REFUNDED', refundId: refund.id },
    });

    await tx.returnRecord.update({
      where: { id: order.returnRecord.id },
      data: { status: 'REFUNDED' },
    });
  });
}

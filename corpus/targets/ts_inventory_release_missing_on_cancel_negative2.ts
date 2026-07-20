// SAFE: Uses Prisma in a transaction to atomically cancel the order and release stock

export async function cancelOrder(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({ where: { id: orderId } });
    if (!order) throw new Error('Order not found');

    if (order.status === 'CANCELLED') {
      throw new Error('Order is already cancelled');
    }

    await tx.product.update({
      where: { id: order.productId },
      data: {
        stock: { increment: order.quantity },
        reserved: { decrement: order.quantity },
      },
    });

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'CANCELLED' },
    });

    if (order.stripePi) {
      await stripe.refunds.create({ payment_intent: order.stripePi });
    }
  });
}

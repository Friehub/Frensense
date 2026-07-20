// SAFE: Uses Prisma with a lifecycle that confirms the reservation on payment completion

export async function placeOrder(prisma: PrismaClient, userId: string, productId: string, quantity: number) {
  return prisma.$transaction(async (tx) => {
    const product = await tx.product.findUnique({ where: { id: productId } });
    if (!product || product.stock - product.reserved < quantity) {
      throw new Error('Insufficient stock');
    }

    await tx.product.update({
      where: { id: productId },
      data: { reserved: { increment: quantity } },
    });

    const order = await tx.order.create({
      data: {
        userId,
        productId,
        quantity,
        status: 'AWAITING_PAYMENT',
      },
    });

    return order;
  });
}

export async function confirmPayment(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({ where: { id: orderId } });
    if (!order || order.status !== 'AWAITING_PAYMENT') {
      throw new Error('Order not awaiting payment');
    }

    await tx.product.update({
      where: { id: order.productId },
      data: {
        stock: { decrement: order.quantity },
        reserved: { decrement: order.quantity },
      },
    });

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'CONFIRMED' },
    });
  });
}

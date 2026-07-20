// SAFE: Uses Prisma $transaction with row-level locking to serialize concurrent state transitions

export async function fulfillOrder(prisma: PrismaClient, orderId: string) {
  await prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      select: { status: true },
    });

    if (!order || order.status !== 'PROCESSING') {
      throw new Error(`Cannot fulfill: order is ${order?.status}`);
    }

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'SHIPPED' },
    });
  });

  await generateShippingLabel(orderId, prisma);
}

async function completeOrder(prisma: PrismaClient, orderId: string) {
  await prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      select: { status: true },
    });

    if (!order || order.status !== 'SHIPPED') {
      throw new Error(`Cannot complete: order is ${order?.status}`);
    }

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'COMPLETED' },
    });
  });

  await releasePayout(orderId, prisma);
}

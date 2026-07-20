// SAFE: Uses Prisma with a pending_points field that only credits on delivery

export async function placeOrder(prisma: PrismaClient, userId: string, total: number) {
  const points = Math.floor(Number(total) * 0.1);

  const order = await prisma.order.create({
    data: {
      userId,
      total,
      status: 'PENDING',
      pendingPoints: points,
    },
  });

  return order;
}

export async function deliverOrder(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({ where: { id: orderId } });
    if (!order || order.status !== 'SHIPPED') throw new Error('Order cannot be delivered');

    await tx.user.update({
      where: { id: order.userId },
      data: { loyaltyPoints: { increment: order.pendingPoints } },
    });

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'DELIVERED', pendingPoints: 0 },
    });
  });
}

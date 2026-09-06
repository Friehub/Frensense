// SAFE: Uses Prisma to join order to reservation and release the correct record

export async function cancelOrder(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      include: { reservation: true },
    });

    if (!order) throw new Error('Order not found');

    if (order.reservation && !order.reservation.released) {
      await tx.inventoryReservation.update({
        where: { id: order.reservation.id },
        data: { released: true },
      });

      await tx.product.update({
        where: { id: order.productId },
        data: {
          stock: { increment: order.quantity },
          reserved: { decrement: order.quantity },
        },
      });
    }

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'CANCELLED' },
    });
  });
}

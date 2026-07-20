// SAFE: Uses Prisma with deferred commission that triggers only on completion

export async function completeOrder(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      include: { affiliate: true },
    });

    if (!order || order.status !== 'DELIVERED') throw new Error('Order not delivered');

    if (order.affiliateCode && order.pendingCommission > 0) {
      const affiliate = await tx.affiliate.findUnique({
        where: { code: order.affiliateCode },
      });

      if (affiliate) {
        await tx.affiliate.update({
          where: { id: affiliate.id },
          data: { balance: { increment: order.pendingCommission } },
        });

        await tx.affiliateTransaction.create({
          data: {
            affiliateId: affiliate.id,
            orderId: order.id,
            amount: order.pendingCommission,
            type: 'commission',
          },
        });
      }
    }

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'COMPLETED', pendingCommission: 0 },
    });
  });
}

export async function cancelOrder(prisma: PrismaClient, orderId: string) {
  await prisma.order.update({
    where: { id: orderId },
    data: { status: 'CANCELLED', pendingCommission: 0 },
  });
}

// SAFE: Uses Prisma in a transaction to atomically credit the wallet and create the ledger entry

export async function issueRefund(prisma: PrismaClient, userId: string, orderId: string) {
  const order = await prisma.order.findUnique({ where: { id: orderId } });
  if (!order) throw new Error('Order not found');

  await prisma.$transaction(async (tx) => {
    await tx.ledger.create({
      data: {
        userId,
        type: 'REFUND',
        amount: order.total,
        referenceType: 'order',
        referenceId: orderId,
      },
    });

    await tx.wallet.update({
      where: { userId },
      data: { balance: { increment: order.total } },
    });

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'REFUNDED' },
    });
  });
}

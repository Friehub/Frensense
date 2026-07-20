// SAFE: Uses Prisma in a transaction to atomically record all ledger entries and wallet updates

export async function issueRefund(prisma: PrismaClient, buyerId: string, sellerId: string, orderId: string, amount: number) {
  return prisma.$transaction(async (tx) => {
    const txnRef = crypto.randomUUID();

    await tx.ledger.create({
      data: { userId: sellerId, type: 'DEBIT', amount, referenceType: 'refund', referenceId: orderId, txnRef },
    });

    await tx.ledger.create({
      data: { userId: buyerId, type: 'CREDIT', amount, referenceType: 'refund', referenceId: orderId, txnRef },
    });

    await tx.wallet.update({
      where: { userId: buyerId },
      data: { balance: { increment: amount } },
    });

    await tx.wallet.update({
      where: { userId: sellerId },
      data: { balance: { decrement: amount } },
    });

    return { refunded: true, amount };
  });
}
